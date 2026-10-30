import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";

const read = (file) => fs.readFileSync(new URL(file, import.meta.url), "utf8");
function job(file, name) {
  const source = read(`../workflows/${file}`);
  return source.split(`\n  ${name}:\n`)[1]?.split(/\n  [\w-]+:\n/)[0] ?? "";
}

test("automatic CI and releases do not build legacy Windows bundles", () => {
  assert.match(read("./ci-plan.mjs"), /windows_win7_bundle: false/);
  for (const name of ["build-windows-7-offline", "build-windows-7-static-browser"]) {
    assert.match(job("release.yml", name), /\n    if: false\n/);
  }
  const cleanup = job("release.yml", "cleanup-release-signatures");
  assert.match(cleanup, /needs: \[build, build-windows\]/);
});

test("DuckDB Windows CI uses the supported stable target", () => {
  const driver = job("ci.yml", "duckdb-windows-driver");
  assert.match(driver, /RUSTUP_TOOLCHAIN: 1\.99\.1/);
  assert.match(driver, /--target x86_64-pc-windows-msvc/);
  assert.doesNotMatch(driver, /nightly|build-std|win7|Windows 7/);
  assert.match(driver, /validate_windows_pe_dependencies\.py/);
  assert.match(driver, /"\$DRIVER" < \/dev\/null/);
});

test("Windows agent release matrices and Go compilers use modern targets", () => {
  const source = read("../workflows/agents-release.yml");
  assert.doesNotMatch(source, /target: x86_64-win7-windows-msvc|win7: true|go-version: "1\.20\.14"/);
  for (const name of ["build-duckdb-native", "build-tdengine-native"]) {
    const block = job("agents-release.yml", name);
    assert.match(block, /target: x86_64-pc-windows-msvc/);
    assert.match(block, /platform: windows-x64/);
  }
  assert.match(source, /go-version: "1\.27\.3"/);
});
