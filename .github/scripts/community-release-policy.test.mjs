import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";

const workflow = fs.readFileSync(new URL("../workflows/publish-packages.yml", import.meta.url), "utf8");

test("package publication does not restore removed community broadcasts", () => {
  assert.doesNotMatch(workflow, /notify-qq:|GroupMessage:|ASTRBOT_API_KEY/);
});

test("upstream distribution roots cannot write from a fork", () => {
  for (const name of ["publish", "upload-r2"]) {
    const block = workflow.split(`\n  ${name}:\n`)[1]?.split(/\n  [\w-]+:\n/)[0] ?? "";
    assert.match(block, /^    if: github\.repository == 't8y2\/dbx'\s*$/m, name);
  }
});

test("desktop locales do not retain unused community contact labels", () => {
  const root = new URL("../../apps/desktop/src/i18n/locales/", import.meta.url);
  for (const file of fs.readdirSync(root).filter((file) => file.endsWith(".ts"))) {
    assert.doesNotMatch(fs.readFileSync(new URL(file, root), "utf8"), /^\s+(?:qqGroup|feishuGroup|wechatGroup|wechatGroupInvite):/m, file);
  }
});

test("removed community QR codes and logos are not shipped", () => {
  for (const file of ["docs/qq-group.jpg", "docs/wechat-group.jpg", "docs/public/icons/community/qq.png", "docs/public/avatars/linuxdo.png"]) {
    assert.equal(fs.existsSync(new URL(`../../${file}`, import.meta.url)), false, file);
  }
});
