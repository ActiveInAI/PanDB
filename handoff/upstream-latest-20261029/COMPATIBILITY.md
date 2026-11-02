# 依赖兼容性决策（2026-11-02）

用户要求依赖优先最新稳定版并保留项目能力。Windows 桌面支持目标为 Windows 11 及以上；Windows 7/10 不再作为适配、验证或验收目标，不再自动构建 Win7 安装包。存在可复现API、ABI、旧格式或操作系统约束的组件列明如下；这些例外意味着不能宣称每个依赖都采用其注册表最新大版本。

| 组件 | 采用版本/来源 | 必须保留的能力及证据 |
| --- | --- | --- |
| 桌面TypeScript | 官方维护兼容包 `npm:@typescript/typescript6@6.0.2` | 最新Vue3.5.44/vue-tsc3.3.13依赖旧JS Compiler API；完整typecheck/build/test已通过；docs独立使用TS7 |
| GTK Rust bindings | 0.18.2 / GLib0.18.5安全回移 | 当前Tauri/Wry/WebKitGTK统一此ABI；GTK0.19导致gdk-3重复links；GLib官方PR1343回移、release安全回归通过 |
| MySQL | mysql_async0.37.0固定兼容fork4a9e60a6 | 最新注册表0.37.1缺少deprecate_eof与旧认证/证书兼容扩展；主工作区和DuckDB独立测试工作区使用一致补丁 |
| PostgreSQL | 固定GaussDB兼容fork ac8ad476 | 保留GaussDB与现有协议扩展；未将源码不同的上游fork HEAD盲目替换 |
| SQL Server | tiberius0.12.3 + 自有补丁 | 注册表0.13缺少send_attention、旧TDS、损坏UTF16恢复；使用当前OpenSSL保留功能 |
| 本机更新器 | 2.10.1 + 安全扩展，ZIP9 | 注册表2.14缺package_kind、签名离线restore_update与受控安装接口；移除其旧ZIP4间接依赖 |
| Kafka旧broker支持 | Java client3.9.2维护线 | 4.4移除alterConfigs，现有不支持incrementalAlterConfigs的broker需此fallback；107项unit通过 |
| MongoDB旧服务兼容 | mongodb-driver-sync3.12.14 | 单独旧服务驱动使用维护兼容线，现代MongoDB路径独立 |
| IRIS / GBase厂商发布线 | intersystems-jdbc3.11.0 / gbasedbt jdbc3.6.5.21 | 保留厂商提供的现有支持线，避免用不对应的Maven变体替换数据库协议 |
| Java agents运行目标 | JRE21维护线 | Java21是既有agent目标；已使用JDK21.0.13验证，Gradle9.8.2为当前构建线 |
| H2旧文件格式 | V1=1.4.200，V2=2.1.214，V3=2.5.252 | 旧格式由隔离引擎只读识别，不能全部换为最新版而丢失读取能力；52项unit通过 |
| 原生加密SQLite | MultipleCiphers2.6.1 + 官方SQLite3.54.1维护补丁，wrapper1.540100.0 | 保留稳定2.6.1密码实现，移植官方维护提交30f1abc0的3.54.1内核；官方SHA3匹配、全部加密钩子计数不变，七种旧加密文件读回/wrong-key/SHA验证通过 |

新组件主线：Node26.13.0 LTS、pnpm12.16.1、Rust1.99.1、Gradle9.8.2；Wry0.57.1和WebView2 sys0.39.1已迁移并保留自有恢复、剪贴板；历史Win7 loader素材仅保留归档，不纳入支持承诺。

Windows agents 与标准目标统一：Go 1.27.3、Rust 1.99.1、`x86_64-pc-windows-msvc`；Win7 发布作业停用，其余发布任务不依赖这些停用作业。

Pageant 已迁移至官方当前稳定版 0.2.5（注册表 API 与归档 SHA256 已核对），不再因 Win7 固定旧版。保留映射边界保护及现代 Windows 账户名规范化；Windows 目标与严格 Clippy、本机账户名 8 项回归已通过；[CI41051741668](https://github.com/ActiveInAI/PanDB/actions/runs/41051741668) 的 Windows 原生 13 项全部通过。

2026-11-02 补充：Windows DuckDB 的 cc-rs 会隐式继承 RUSTC_WRAPPER=sccache；sccache0.16 重排分离的 -Tp 与 /WX-，导致编译失败。此任务使用按工具链/锁文件/MSVC/SDK区分的原生产物缓存，不在该 C++ 路径启用编译包装器；主分支实际 Windows SQL/Parquet/重开四项已通过。普通 Rust/C 编译路径保留现有缓存。

Vite 未使用的旧 Stylus/glob/minimatch 可选链已从实际应用依赖图清除；可选版本约束为 Stylus0.64.0/glob14.0.3/minimatch11.0.3。隔离工程已验证这组版本与项目 Vite8.3.5 的 glob 样式导入；应用未新增必需编译器依赖。仅与已移除 source-map-resolve0.6 对应的过期补丁及规则一并清理，未关闭补丁、完整性或漏洞检查。
