# 安全复查（2026-10-30）

本次没有添加审计忽略项，没有关闭权限、证书、签名、主机密钥或安装包校验。原始module级清单见security-inventory.json。

| 范围 | 观察结果与处置 |
| --- | --- |
| npm根工作区、文档、dev-host | 当前三套锁定依赖审计均零告警 |
| 旧Rust TLS链 | 删除AWS默认旧HTTPS后端及旧pemfile；MQTT明确安全最低版本；SQL Server保留项目补丁并使用当前OpenSSL |
| ZIP / LRU | 主图及独立packager升级ZIP9；主图和DuckDB独立测试图使用已修复LRU0.18.5 |
| GLib RUSTSEC-2024-0429 | 保留GTK0.18 ABI，回移官方PR1343到VariantStrIter::impl_get。校验官方crate SHA后，以其未修复源码进行release回归，复现SIGSEGV；项目修复后相同测试通过 |
| Pageant CVE-2026-102820 | 本地旧分支确有未检查的共享内存读取。已补checked-add/8192字节帧上限、SID存储所有权及安全属性借用生命周期；Windows target编译和严格Clippy通过，真实Windows内核mapping release回归及既有用例13项通过 |
| 原生SQLite | 稳定MultipleCiphers2.6.1保持不变，内核移植官方3.54.1维护补丁；已核对官方SHA3和全部加密钩子，七种旧加密格式读回/wrong-key/只读SHA验证通过 |
| block-buffer GHSA-qwgh-2vcv-g2f7 | 保留registry告警记录。0.10.4的cursor独立于block数据，源码没有被报告的ReadBuffer；公开panic场景本地验证cursor不变且后续digest正常，不能把针对inline cursor的缺陷直接映射到此实现 |
| RSA RUSTSEC-2023-0071 | 上游明确无已修复版本，当前russh/ssh-key仍依赖它；保留为上游残余风险，不能宣称已修复或全Rust零漏洞 |
| Rust维护状态 | proc-macro-error、unsafe-libyaml、zvariant相关、paste、scc等仍有维护状态告警。它们与已证实可达CVE不同，未以隐藏告警替代修复，也未为消除元数据提示删掉既有功能 |
| Go | 21个模块在当前Go1.27.3下build与short unit通过；实际驱动和基准工具的可升级依赖已更新。module构建列表仍含未进入生产import图的历史工具/测试依赖。x/crypto0.58.0为官方当前latest，其OpenPGP弃用告警未在生产导入中命中 |
| 凭据与旧数据 | Argon2 0.6验证旧0.5 PHC成功，错误密码拒绝；七种密码算法的新SQLite3.54读取旧3.51.3加密文件，错误key拒绝，只读前后SHA一致 |

本地及GitHub验证不等于所有真实数据库、供应商环境、旧Windows主机的用户验收。未对用户真实数据库执行迁移、未更改凭据和服务，未发布安装包。

来源：[GLib公告](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)、[官方修复](https://github.com/gtk-rs/gtk-rs-core/pull/1343)、[RSA公告](https://rustsec.org/advisories/RUSTSEC-2023-0071.html)、[block-buffer公告](https://osv.dev/vulnerability/GHSA-qwgh-2vcv-g2f7)、[Go OpenPGP公告](https://pkg.go.dev/vuln/GO-2026-5932)。
