# 上游同步与依赖升级状态：未完成，2026-10-29

当前未提交、未推送、未部署。原 main、生产应用、系统包与凭据未改动。

独立工作树：`/home/arch-gpt--notebook001/dev/worktrees/pandb/upstream-latest-dependencies-20261029`

分支：`maintenance/upstream-latest-dependencies-20261029`

PanDB 基准：`1dfead3cbaa6a181230ed6bf19152f7e748192ca`。
上游最新快照：`1b8dfd9673e993f65a6253c0faa0882bd49a544b`。
上游架构迁移超过 6000 个文件。合并预览已在独立工作树展开，不能视为合并验收完成。

## 已通过的检查

| 检查 | 观察结果 |
| --- | --- |
| GitHub 脚本 `node --test .github/scripts/*.test.mjs` | 314/314 通过，exit 0 |
| 文档 `next build` | 使用最新依赖完成生产静态构建，exit 0 |
| 文档 postbuild | 266 个索引页面、270 个本地化页面、1486 个导出文件通过验证 |
| 文档 SEO 测试 | 10/10 通过，exit 0 |
| 文档内容检查 | 42 篇双语页面通过 |
| 中文搜索实际查询 | 最新 ZBSearch 4.0.1 + 原有中文 tokenizer，查询“数据库”命中“生产数据库只读安全” |
| 工作区 npm audit | 0 info / low / moderate / high / critical，exit 0 |
| 文档站 npm audit | 0 info / low / moderate / high / critical，exit 0 |
| Go 模块 | 21/21 `go build ./...` 通过，未执行真实数据库集成测试 |
| 前端格式检查 | 3887 个文件通过，exit 0 |
| 前端 lint | exit 0，仍有 warning，不能声称零问题 |
| 社区渠道回归 | 清除上游新增波兰语 README 与首页带回的渠道内容，目标字符串扫描通过；排除第三方 vendor 和既有通知 workflows |

Go 的首次 main 包构建定位到用户根目录的 Git 仓库而失败。修正为命令级 `GIT_DIR` / `GIT_WORK_TREE` 指向真实任务工作树后，16 个受影响模块全部通过；没有关闭 VCS stamping。其余 5 个库模块原本通过。

## 已做的兼容性与安全修复

- 升级官方 registry 已核对的 npm / Rust 直接依赖；Go 11 份清单更新并重新 tidy / 构建。
- 本地使用 pnpm 12.16.1 与 Rust 1.99.1；不改全局默认工具链，不安装系统包。
- 将 Keyring 4 的 flags 和调用迁至 v1；Reqwest 0.13 使用新的 `rustls` flag。Rust 整体仍未编译通过。
- 保留 PanDB 产品身份、环境变量 aliases、手动文档部署策略及社区信息删除。
- 取消仅用于 CSS 的 shadcn-vue 生成器依赖，保留原样式及 MIT 许可；清除其携带的漏洞链。
- 将 source-map-resolve 的解码器升级到修复版 0.5.1，增加 CommonJS / ESM 兼容补丁；实际 URL 解码和 100 KB 畸形输入检查通过。
- 文档站从旧 Orama 接口迁至 Fumadocs 当前的 ZBSearch，保留中文搜索。
- 适配 Fumadocs 新 authored-source Markdown 接口、Lucide 图标重命名；GitHub 品牌图标复用已有 SVG。

## 不能宣称完成的原因

1. **所有依赖强制最新与现有桌面生态不兼容**：最新 Tauri 2.12.5 / Wry 0.56.1 / WebKitGTK 2.0.2 仍依赖 GTK 0.18；直接升级 GTK 0.19.0 后，Cargo 拒绝同时链接两个版本的 gdk-3。证据：`cargo-lock-update-reqwest-v13.log` 和本地 crates.io sparse index。
2. **TypeScript 7 与最新 Vue 工具链不兼容**：TypeScript 7.0.2 不再导出旧 `typescript/lib/tsc` 和 JS Compiler API；vue-tsc 3.3.13 与 Vue compiler-sfc 3.5.44 仍依赖旧接口。前端 typecheck、组件测试和桌面生产构建失败。没有降级、屏蔽检查或加 vue-ignore。
3. 完整前端测试仍失败；其中还出现 Happy DOM / CodeMirror 嵌套更新问题，需在类型工具链策略确定后逐项复现修复。
4. Rust vendor / Git 补丁尚未移植到所有最新版：SQL Server ATTENTION、SQLite 多加密、PostgreSQL GaussDB、MQTT 等不能盲目撤销旧安全补丁。Java / Gradle 仅完成版本调查，尚未升级和构建。
5. 全仓 `git diff --check` 尚未通过，主要包含上游 BAT 的 CRLF / whitespace 报告；未盲目重写上游 Windows 文件。
6. 新增/迁移 workflows 还需完整审查，不能意外重新开启旧上游生产发布、镜像同步或外部通知。

## 等待用户明确的兼容性策略

已通过异步问题请求，但尚未收到答案：是否允许受上游生态约束的依赖使用可兼容的维护版本，并逐项记录例外；以及旧平台 / 旧驱动支持是否保留。

在答复前，不继续通过降低版本解决上述冲突，不发布不通过的工作树。

## 后续步骤

- 用户决定策略后，修复 Rust 解析 / 特性 / API，移植 vendor 与 Git 补丁，运行 fmt、clippy、核心/CLI/MCP/Web/桌面测试。
- 采用获准的 Vue 类型工具链策略，修复完整前端测试和生产构建。
- 核对 Maven metadata 的 release 字段（不能把 versions 列表末尾旧分支当最新版），再升级 Java / Gradle 并验证驱动。
- 重跑受影响审计和回归，复核品牌/更新来源/禁用发布边界，再形成可审阅提交。
- 新一轮 GitHub 推送需对具体目标、范围和回滚方案确认；此前社区删除的授权不可复用。

主要命令日志在 `/tmp/pandb-upgrade-20261029/`，可审阅结果保存在本目录 `verification-summary.json`、`dependency-versions.json` 和 `CHECKPOINT.md`。本报告不是“全部问题已修复”的证明。
