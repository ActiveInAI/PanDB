# 兼容性决策（2026-10-30）

用户已要求所有中间决策由 Agent 负责，仅做最终验收。依赖优先最新稳定版；有可复现上游约束时采用仍维护的兼容版本并记录。

| 项目 | 采用版本 | 原因与验证 |
| --- | --- | --- |
| 桌面 TypeScript | npm:@typescript/typescript6@6.0.2 | 官方维护的 TS6 兼容包；Vue 3.5.44 / vue-tsc 3.3.13 依赖旧 JS Compiler API。文档站独立保留 TS7。待 typecheck / build / test。 |
| GTK Rust bindings | 0.18.2 | 最新 Tauri/Wry/WebKitGTK 仍统一要求 gtk/gdk 0.18；0.19 会发生 gdk-3 链接冲突。待 Cargo 编译与测试。 |

| MySQL async | 0.37.0 + pinned compatibility fork | 0.37.1 lacks the deprecate_eof option used to preserve per-result-set server messages. The existing fork is required; validate stored-procedure/multi-result handling. |

| Kafka legacy broker agent | 3.9.2 | Kafka 4.4 removes alterConfigs, which is required by the current explicit fallback for brokers lacking incrementalAlterConfigs. Keep the latest 3.9 maintenance release. |

| SQL Server native driver | 0.12.3 + maintained project patches | Registry0.13 does not expose send_attention and lacks the existing legacy-TDS/Unicode recovery extensions. Keep the actual patched driver and current vendored OpenSSL, rather than silently discarding cancellation and older-server support. |

| Native updater | 2.10.1 + project security extensions | Retains package_kind, signed offline restore_update, runtime-only installer paths and checked Windows process launch. Registry2.14 lacks these existing interfaces. |
| Windows Pageant bridge | 0.2.1 + project Win7/SID fixes | Retains existing compatible owner-security and plain Win32 string handling. |
