# PanDB 同步与依赖升级验收（2026-10-30）

交付目标：`ActiveInAI/PanDB/main`。上游基线：`6bac6034b5b3e17780d49b80363db026b6a7003e`。功能源码冻结于 `f2a5f0b8`；`a1be700a` 增强 Nix 依赖验证，已比较其功能源码字节一致。最后的插件发布模板默认值与项目 Node26.13.0 / Go1.27.3 / CLI0.1.9 对齐，321项工作流脚本再次通过。

## 观察到的通过结果

| 验证 | 结果 |
| --- | --- |
| Rust 工作区 nextest | **11526通过、0失败、370按原有规则跳过**；8MiB栈，line-tables-only调试信息 |
| Rust严格检查 | CI全工作区、all-targets、`-D warnings`通过 |
| Linux桌面 | 默认功能完整native build/link通过，包含最新3.54.1多密码SQLite |
| Windows / macOS / Nix | [交付验收CI41043514043](https://github.com/ActiveInAI/PanDB/actions/runs/41043514043)全部成功；Nix实际构建pnpm包、JS依赖、Rust源vendor并验证Git固定哈希 |
| Windows Pageant | 真实内核mapping release回归及既有用例合计13通过，包括恶意长度、边界与SID生命周期 |
| 前端 | Node26下format/lint/typecheck/全量test及生产build通过 |
| 文档 | Node26生产build及postbuild通过；266索引页、270本地化页、1486导出文件 |
| GitHub脚本 | 321通过，含3项现代Windows支持策略回归和4项保留手工/停用配置选择检查 |
| Go | Go1.27.3下21模块build和short unit命令通过 |
| Java | 持久JUnit报告读回527通过，无失败和跳过 |
| 插件SDK | Node26开发宿主build及58测试通过；Rust CLI23、packager8通过 |
| SQL / 方言 | SQL-core713、原SQL分析集成case46、方言173通过；37份内置catalog均可加载 |
| DuckDB | 实际native2.0.0执行BIGINT/中文/DECIMAL，Parquet写读与关闭重开通过 |
| 旧数据兼容 | 七种3.51.3加密fixture在实际Cargo产出的3.54.1库中只读读回、拒绝wrong-key，前后SHA一致；Argon2新版本验证旧PHC通过 |
| 实际界面 / HTTP / MCP | 浏览器登录、连接、编辑和执行SQL、Canvas中文结果视觉核对通过；HTTP8项、MCP4项，包括显式只读拒写 |

## 已修复的关键问题

保持PanDB显示名称和应用更新来源，保留内部DBX协议及历史PanDB错误前缀解析。修正依赖API迁移、TLS/JWT后端选择、代理registry来源、SQL解析器适配、ZIP尺寸保护、编辑器事件调度与负时间格式。补回GLib和Pageant安全修复，校正Windows SID与安全属性的内存生命周期。多密码实现保留稳定2.6.1，内核补齐官方SQLite3.54.1维护修复，来源哈希与codec钩子已核对。

## 验收边界

这是源码、依赖、构建与隔离功能验证；未对真实业务数据库或旧Windows实机执行迁移和运行验收。370项平台/显式集成用例按原配置跳过。

**不能宣称每个依赖均为其注册表最新大版本，也不能宣称全项目零漏洞。** API/ABI与旧格式的维护兼容线见[兼容性说明](COMPATIBILITY.md)；RSA上游无已修复版本及其它维护状态告警、Go未使用依赖图告警的分类见[安全复查](SECURITY-REVIEW.md)。未添加审计忽略项。

## 文档交付与证据

DOCX绝对路径：`/home/arch-gpt--notebook001/Downloads/PanDB_已删除社区相关内容.docx`。ZIP完整性与逐段社区词/链接复查均通过。

结构化结果见[verification-summary.json](verification-summary.json)。原始日志与隔离验收文件位于 `/home/arch-gpt--notebook001/work/pandb-upgrade-20261030`；本地开发工作树为 `/home/arch-gpt--notebook001/dev/worktrees/pandb/upstream-latest-dependencies-20261029`。

2026-10-30 支持策略调整：Windows 桌面以 Windows 11+ 为目标；停用 Win7 自动安装包与发布任务，保留标准 Windows 检查，DuckDB/TDengine Windows x64 使用标准 Rust 1.99.1 目标，Go 发布构建统一到 1.27.3。321 项脚本测试全通过，3 个 YAML 与 58 段 Bash 语法验证通过；应用功能代码未改变。
