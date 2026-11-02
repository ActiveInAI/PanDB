# 安全复查（2026-11-02）

本轮未添加审计忽略项，未关闭证书、主机密钥、签名、安装包或补丁校验。完整版本清单及逐项结果见 [security-inventory.json](security-inventory.json)。GitHub 告警数量与实际代码风险分别记录。

| 范围 | 观察结果与处置 |
| --- | --- |
| npm 根工作区、文档、开发宿主 | 2026-11-02 分别使用各自锁文件对应的 pnpm/npm 审计，当前均零告警 |
| 可选样式依赖 | 根工作区旧 Stylus/glob/minimatch 可选链已从实际依赖图清除；安全版本约束为 0.64.0 / 14.0.3 / 11.0.3，隔离工程验证与项目 Vite8.3.5 的 glob 导入兼容，15包审计0。应用未新增必需编译器依赖；过期 source-map-resolve 补丁随依赖退出而移除，严格补丁检查保留 |
| Rust registry | 重新查询全部7个锁文件、1053个包版本；仍有下述上游风险和维护状态记录，不能宣称全Rust零漏洞 |
| GLib RUSTSEC-2024-0429 | 保留 GTK0.18 ABI，回移官方PR1343到 VariantStrIter::impl_get；校验官方crate SHA后，原始源码release回归复现SIGSEGV，项目补丁同测通过 |
| Pageant CVE-2026-102820 | 已升级官方0.2.5，校验SHA256及Apache-2.0许可；移除Win7绑定与自管SID内存，保留映射长度、游标溢出、空view及可变读目标保护。Windows目标/严格Clippy、本机账户名8项及真实Windows release13项通过 |
| 独立AI代理工具 | 同步遗漏的独立锁文件，Rustls0.23.46 / WebPKI0.103.16；独立图OSV无命中，已在标准Windows CI实际编译验证 |
| 原生SQLite | 稳定MultipleCiphers2.6.1密码实现，内核移植官方3.54.1维护补丁；官方SHA3与全部加密钩子匹配，七种旧加密文件正确读回、错误key拒绝、只读SHA不变 |
| ZIP / LRU | 主图和独立打包器使用ZIP9，主图和DuckDB测试图使用已修复LRU0.18.5；尺寸保护与异常路径回归通过 |
| block-buffer | 0.10.4源码无被报告的inline ReadBuffer实现，公开panic场景验证cursor不变、后续digest正确；本次最新OSV查询已不再命中此前的过宽范围映射 |
| RSA RUSTSEC-2023-0071 | 官方公告仍明确0.10.0-rc.18受影响且无已修复版本。普通依赖反向图经russh0.65.0/ssh-key0.7.0-rc.11引入；保留为上游残余风险，未删除现有SSH能力或隐藏告警 |
| Rust维护状态 | proc-macro-error、unsafe-libyaml、zvariant相关、paste、scc仍有维护状态告警，原始ID完整保留；不以更改版本号或忽略配置消除提示 |
| Go | 21模块在Go1.27.3下build/short unit通过；module列表仍含未进入生产import图的历史工具/测试依赖，x/crypto OpenPGP弃用未在生产导入中命中 |
| 发布边界 | 删除QQ群广播任务及未引用的社区二维码/图标；固定指向上游Homebrew/Scoop/R2目标的两根发布作业限定在上游仓库运行。未替用户创建或使用其它发布目标 |
| 凭据与旧数据 | Argon2新版本验证旧PHC及错误密码拒绝通过；隔离HTTP/MCP验证认证拒绝与显式只读拒写，未使用真实业务数据库或迁移用户凭据 |

Windows DuckDB构建失败来自cc-rs隐式继承RUSTC_WRAPPER与sccache0.16的MSVC参数重排；使用按锁文件、Rust工具链、MSVC/SDK环境隔离的原生产物缓存，保持实际编译和运行检查。该路径已通过Windows SQL、中文/DECIMAL、Parquet、checkpoint与重开验收。

本地及CI证据不等于所有供应商环境的业务验收。真实数据库迁移、安装包发布和生产部署不属于此次执行范围。源代码兼容性例外见 [COMPATIBILITY.md](COMPATIBILITY.md)。

来源：[GLib公告](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)、[官方修复](https://github.com/gtk-rs/gtk-rs-core/pull/1343)、[RSA公告](https://rustsec.org/advisories/RUSTSEC-2023-0071.html)、[Pageant官方包](https://crates.io/crates/pageant/0.2.5)、[WebPKI公告](https://rustsec.org/advisories/RUSTSEC-2026-0382.html)、[minimatch公告](https://github.com/advisories/GHSA-g3hv-75wg-m249)、[Go OpenPGP公告](https://pkg.go.dev/vuln/GO-2026-5932)。
