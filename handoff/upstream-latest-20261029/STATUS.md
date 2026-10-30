# PanDB 上游同步与依赖升级：2026-10-30

维护分支已更新到 GitHub；主分支尚未更新。用户已委托中间实现与验证，只做最终验收。

- 工作树：`/home/arch-gpt--notebook001/dev/worktrees/pandb/upstream-latest-dependencies-20261029`
- 分支：`maintenance/upstream-latest-dependencies-20261029`
- 冻结上游：`6bac6034b5b3e17780d49b80363db026b6a7003e`
- 持久工具、日志与验收数据：`/home/arch-gpt--notebook001/work/pandb-upgrade-20261030`

| 验证 | 当前观察结果 |
| --- | --- |
| 前端 run-check | Node24与Node26均5项全部通过；包含全部Vitest测试 |
| 前端生产构建 | Node26通过 |
| 文档站 | Node26生产build、postbuild通过；266索引页、270本地化页、1486文件 |
| GitHub脚本 | 最新工作流调整后314/314通过；YAML无重复键 |
| Go | Go1.27.3下21/21模块build和short unit通过 |
| Java | 持久JUnit XML读回common368、H2 52、Kafka107，合计527通过无失败/跳过 |
| 插件 | CLI23测试通过；packager测试通过；示例check通过 |
| GLib | release优化下out-pointer安全回归通过 |
| 加密SQLite | 新3.54读取3.51.3旧加密fixture、错误key拒绝、只读SHA不变 |
| 默认桌面原生程序 | 完整cargo build已观察成功；后续内部错误Box化后待最终验证 |
| Wry Windows target | 本机隔离cargo check通过，无warning |
| Nix | GitHub维护验证两轮Nix成功；取回验证后的pnpm依赖hash |
| Web/浏览器/MCP | 先前实际登录、SQL编辑执行、Canvas表格中文数据验收通过；最新原生二进制构建及持久证据收口中 |
| Rust Clippy / workspace nextest | 严格全target检查、全量测试尚在收口；不能报告全部通过 |
| Windows完整desktop check | GitHub维护run41027625666运行中 |

npm审计已观察根目录、文档、dev-host零告警；Rust和Go的module级告警含无修复/维护状态/未使用包，按最终安全台账逐项列明，不能用npm零告警替代全项目安全结论。兼容版本决策见COMPATIBILITY.md。

当前未部署、未发布安装包，未改变真实数据库/凭据/系统服务。保留原fork最新contributors更新；不会force push。
