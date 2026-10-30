import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";

const workflows = ["ci.yml", "release.yml", "agents-release.yml", "sqlserver2000-windows-user-test.yml"];
for (const file of workflows) {
  const source = fs.readFileSync(new URL(`../workflows/${file}`, import.meta.url), "utf8");
  const jobs = source.slice(source.indexOf("\njobs:\n") + 7);
  const starts = [...jobs.matchAll(/^  ([A-Za-z0-9_-]+):\s*$/gm)];
  for (let index = 0; index < starts.length; index += 1) {
    const match = starts[index];
    const block = jobs.slice(match.index, starts[index + 1]?.index ?? jobs.length);
    if (!block.includes("dtolnay/rust-toolchain@nightly")) continue;
    test(`${file}/${match[1]} explicitly selects nightly over the project toolchain`, () => {
      const selection = block.match(/^      RUSTUP_TOOLCHAIN: (.+)$/m)?.[1];
      assert.ok(selection, "The job must override the root rust-toolchain.toml");
      assert.match(selection, /nightly-\d{4}-\d{2}-\d{2}/);
      if (block.includes("if: matrix.win7 == true")) {
        const projectChannel = fs.readFileSync(new URL("../../rust-toolchain.toml", import.meta.url), "utf8").match(/channel = "([^"]+)"/)?.[1];
        assert.ok(selection.includes("matrix.win7 == true"));
        assert.ok(selection.endsWith(`|| '${projectChannel}' }}`), "Modern matrix entries retain the project stable toolchain");
      } else {
        assert.match(selection, /^nightly-\d{4}-\d{2}-\d{2}$/);
      }
      assert.ok(block.includes("toolchain: ${{ env.RUSTUP_TOOLCHAIN }}"));
    });
  }
}
