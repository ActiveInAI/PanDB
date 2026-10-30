# 依赖兼容性决策（2026-10-30）

用户要求依赖优先最新稳定版并保留项目能力。存在可复现API、ABI、旧格式或操作系统约束的组件列明如下；这些例外意味着不能宣称每个依赖都采用其注册表最新大版本。

| 组件 | 采用版本/来源 | 必须保留的能力及证据 |
| --- | --- | --- |
| 桌面TypeScript | 官方维护兼容包 `npm:@typescript/typescript6@6.0.2` | 最新Vue3.5.44/vue-tsc3.3.13依赖旧JS Compiler API；完整typecheck/build/test已通过；docs独立使用TS7 |
| GTK Rust bindings | 0.18.2 / GLib0.18.5安全回移 | 当前Tauri/Wry/WebKitGTK统一此ABI；GTK0.19导致gdk-3重复links；GLib官方PR1343回移、release安全回归通过 |
| MySQL | mysql_async0.37.0固定兼容fork4a9e60a6 | 最新注册表0.37.1缺少deprecate_eof与旧认证/证书兼容扩展；主工作区和DuckDB独立测试工作区使用一致补丁 |
| PostgreSQL | 固定GaussDB兼容fork ac8ad476 | 保留GaussDB与现有协议扩展；未将源码不同的上游fork HEAD盲目替换 |
| SQL Server | tiberius0.12.3 + 自有补丁 | 注册表0.13缺少send_attention、旧TDS、损坏UTF16恢复；使用当前OpenSSL保留功能 |
| 本机更新器 | 2.10.1 + 安全扩展，ZIP9 | 注册表2.14缺package_kind、签名离线restore_update与受控安装接口；移除其旧ZIP4间接依赖 |
| Pageant | 0.2.1 + Win7/SID修复 | 保留旧Windows兼容和用户身份检查；不能用升级丢弃这些扩展 |
| Kafka旧broker支持 | Java client3.9.2维护线 | 4.4移除alterConfigs，现有不支持incrementalAlterConfigs的broker需此fallback；107项unit通过 |
| H2旧文件格式 | V1=1.4.200，V2=2.1.214，V3=2.5.252 | 旧格式由隔离引擎只读识别，不能全部换为最新版而丢失读取能力；52项unit通过 |
| Windows7 Go runtime | 现有发行流程专用Go1.20.14 | 最后支持Win7的Go发布线；现代Go模块验证使用当前稳定Go1.27.3；不声称旧系统运行时是当前稳定版 |
| 原生加密SQLite | MultipleCiphers2.6.1 / SQLite3.54.0，wrapper1.540000.0 | 普通3.54.1 SQLite不含多密码引擎，不能直接替换；已官方checksum验证，真实旧加密fixture读回与wrong-key拒绝通过 |

新组件主线：Node26.13.0 LTS、pnpm12.16.1、Rust1.99.1、Gradle9.8.2；Wry0.57.1和WebView2 sys0.39.1已迁移并保留自有恢复、剪贴板与Win7 loader哈希保护。
