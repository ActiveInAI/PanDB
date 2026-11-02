"""Exercise a compiled DuckDB worker over its real stdio protocol on any OS."""

import argparse
from contextlib import contextmanager
from decimal import Decimal
import json
from pathlib import Path
from queue import Queue, Empty
import subprocess
import sys
import tempfile
from threading import Thread


class Worker:
    def __init__(self, executable, directory, stderr):
        self.process = subprocess.Popen(
            [str(executable)], cwd=directory, stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, stderr=stderr, text=True, encoding="utf-8",
        )
        self.responses = Queue()
        self.sequence = 0
        self.reader = Thread(target=self.read_responses, daemon=True)
        self.reader.start()

    def read_responses(self):
        try:
            for line in self.process.stdout:
                self.responses.put(json.loads(line))
        except Exception as error:
            self.responses.put(error)
        finally:
            self.responses.put(None)

    def request(self, method, params):
        self.sequence += 1
        request_id = str(self.sequence)
        self.process.stdin.write(json.dumps({"id": request_id, "method": method, "params": params}) + "\n")
        self.process.stdin.flush()
        try:
            response = self.responses.get(timeout=30)
        except Empty as error:
            raise AssertionError("DuckDB worker response deadline exceeded") from error
        assert isinstance(response, dict), f"Worker closed or returned invalid protocol data: {response!r}"
        assert response.get("id") == request_id, "Worker response ID mismatch"
        assert response.get("ok") is True, response.get("error")
        return response.get("result")

    def connect(self, database):
        self.request("connect", {"path": str(database), "attachedDatabases": [], "initScript": None})

    def query(self, sql):
        return self.request("execute", {
            "database": None, "sql": sql, "maxRows": 100,
            "preserveInsertionOrder": False,
        })

    def shutdown(self):
        self.request("shutdown", {})
        self.process.stdin.close()
        assert self.process.wait(timeout=10) == 0, "Worker did not shut down cleanly"

    def cleanup(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=10)
        self.reader.join(timeout=2)
        self.process.stdout.close()
        if not self.process.stdin.closed:
            self.process.stdin.close()


@contextmanager
def worker(executable, directory, stderr):
    instance = Worker(executable, directory, stderr)
    try:
        yield instance
    finally:
        instance.cleanup()
        if sys.exc_info()[0] is not None:
            stderr.flush()
            print(Path(stderr.name).read_text(encoding="utf-8", errors="replace")[-4000:], file=sys.stderr)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("executable", type=Path)
    args = parser.parse_args()
    executable = args.executable.resolve(strict=True)
    checks = []
    with tempfile.TemporaryDirectory(prefix="pandb-duckdb-smoke-") as directory:
        directory = Path(directory)
        database = directory / "sample.duckdb"
        parquet = directory / "sample.parquet"
        with (directory / "worker-stderr.log").open("w", encoding="utf-8") as stderr:
            with worker(executable, directory, stderr) as client:
                client.connect(database)
                version = client.query("SELECT version() AS version")["rows"][0][0]
                assert isinstance(version, str) and version, "Missing native engine version"
                checks.append("native engine version")
                client.query("CREATE TABLE acceptance(id BIGINT PRIMARY KEY, label VARCHAR, amount DECIMAL(12,2))")
                client.query("INSERT INTO acceptance VALUES (1, '中文原生验收', 1.25)")
                row = client.query("SELECT id, label, amount FROM acceptance")["rows"][0]
                assert row[0] == 1 and row[1] == "中文原生验收" and Decimal(str(row[2])) == Decimal("1.25"), row
                checks.append("BIGINT Unicode DECIMAL")
                parquet_sql = parquet.as_posix().replace("'", "''")
                client.query(f"COPY acceptance TO '{parquet_sql}' (FORMAT PARQUET)")
                assert parquet.is_file(), "Parquet output missing"
                assert client.query(f"SELECT id, label, amount FROM read_parquet('{parquet_sql}')")["rows"][0] == row
                checks.append("Parquet write and read")
                client.shutdown()
            with worker(executable, directory, stderr) as client:
                client.connect(database)
                assert client.query("SELECT id, label, amount FROM acceptance")["rows"][0] == row
                client.shutdown()
                checks.append("checkpoint and persisted reopen")
    print(json.dumps({"engine_version": version, "passed": len(checks), "checks": checks}))


if __name__ == "__main__":
    main()
