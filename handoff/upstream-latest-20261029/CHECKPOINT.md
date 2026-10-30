# PanDB 上游同步与全依赖升级（进行中，2026-10-29）

## 目标与授权

用户要求同步上游，所有组件依赖使用最新版，并全面排查修复。允许本地独立分支合并、依赖/锁文件更新及兼容性修复。尚未获准本轮 GitHub 推送或部署；此前确认仅属于社区信息删除。不得修改系统包、生产服务、凭据或自动启用外部发布。当前没有子代理。

## 位置与基准

- 工作树：`/home/arch-gpt--notebook001/dev/worktrees/pandb/upstream-latest-dependencies-20261029`
- 分支：`maintenance/upstream-latest-dependencies-20261029`
- HEAD / origin main：`1dfead3cbaa6a181230ed6bf19152f7e748192ca`
- 上游最新快照：`1b8dfd9673e993f65a6253c0faa0882bd49a544b`，2026-10-29 Oracle 中文兼容修复。
- 原 upstream remote 只 fetch v0.5.77。显式 fetch 最新 main 成功下载对象，但更新旧 t8y2/main 被 non-fast-forward 拒绝；最新对象保留在 FETCH_HEAD。没有强推或改 remote 配置。
- PanDB 是独立初始化历史。基于旧上游 v0.5.98 (`5c738f5b`) 作显式 merge-base 进行 merge-tree 预览，该版本对应旧导入的 package.json；尚未形成合并提交。

## 已做本地改动

- 将 merge-tree 生成树在干净工作树展开，精确文件清单 `/tmp/pandb-upgrade-20261029/merge-touch-list.tsv`；总体超过 6000 个文件。
- 270 个纯品牌冲突块自动处理，45 份剩余文本冲突按身份/安全门禁保留 ours，模块迁移与功能采用 upstream；移除上游已迁移的旧路径。当前须继续审查，不能视为合并完成。
- 保留 PanDB productName、com.activeinai.pandb、1.0.0；保留社区渠道删除。PANDB 环境变量 aliases 函数保留于 dbx-core，并接入迁移后 CLI/MCP/Web 启动入口。
- 41 份 npm / Cargo 清单已改为 registry 当前稳定版；路径/Git 补丁暂未升级，不满足全部最新版。
- Rust keyring 4 的 flags/API 已迁至 v1；Reqwest 0.13 的 rustls-tls flag 已迁为 rustls，尚未编译验收。
- `.github/scripts/ci-workflow.test.mjs` 正在适配 PanDB 手动文档部署政策，必须防止为了通过上游测试而启用旧上游生产部署。

## 工具与记录

- Python 只能使用 `/home/arch-gpt--notebook001/.venvs/ai/bin/python`。
- 临时分析工具/日志：`/tmp/pandb-upgrade-20261029/`。
- npm 114 / Rust 130 个包的官方 metadata：`dependency-registry-snapshot.json`，旧基准 `dependency-registry-before.json`。
- registry 查询脚本 `inspect-dependencies.py` 会复用缓存，跳过 Cargo templates。
- 直接升级脚本 `update-direct-dependencies.py` 已执行，勿盲目重复覆盖后续兼容性修复。
- pnpm 12.16.1 官方原生包已下载并验证 SHA-512，命令 `/tmp/pandb-upgrade-20261029/pnpm-native/package/pnpm`；只装在临时目录，没有全局安装。
- 本机现有默认 Rust 1.95，已在用户 rustup 安装并验证可运行 `cargo +1.99.1`；不修改默认工具链。下载探针超时后按已证实大包慢速续传完成，日志 rustup-install-resume.log。
- pnpm 12 的默认运行锁目录落在不可写 XDG_RUNTIME_DIR；使用任务专属 XDG_RUNTIME_DIR/TMPDIR 和用户缓存 store，没有放宽沙箱。

## 已观察验证

- 官方 GitHub Dependabot 当前开放 92 条，清单 `dependabot-open.tsv`（非修复后结果）。
- Node GitHub 脚本 314 测试：313 通过，1 失败（上游假设 docs 自动部署；本 fork 故意禁用）。
- Node scripts 98 测试：91 通过、1 失败、6 跳过；失败为旧测试假设 Win7 不用 sccache，而新上游已经采用新版 sccache，需要查清后修复。
- 文档内容检查：42 篇双语文档通过。
- cargo metadata --no-deps 通过；真实依赖解析尚未通过。先后暴露 keyring flags 和 Reqwest TLS flags 迁移问题，已按官方源码修正。
- 不得宣称全面修复、全部最新版、合并完成、发布完成或用户验收。

## 待办与边界

1. 查看运行中的 pnpm 锁文件解析 / cargo 依赖解析结果，继续修复大版本特性差异。
2. 解决 Git / vendor 补丁旧版本冲突；保持 SQL Server/MQ 安全修复，禁止无审查恢复旧弱依赖。
3. Go 有 23 个清单、89 个不同模块；Java 约45个坐标（含模板与旧兼容示例），尚未升级。需要分别核验最新稳定版和构建。
4. 待用户异步偏好：最新版优先移除不兼容旧平台/驱动，还是保留兼容性并列例外。未收到答复，不可声称已获准撤销既有支持。
5. 全仓检查社区渠道残留；上游曾新增设置页 QQ 按钮，保留 ours 已避免部分恢复，仍需全仓复核。
6. 最新上游新增 download 平台模块仍含 DBX/CDN/其他组织链接，需要保留 PanDB identity / 更新来源边界并运行测试。
7. 审查所有新/变更 workflows，不能意外启用自动外部消息/部署/镜像同步。
8. 锁文件确定后 install --ignore-scripts，审查所需 build scripts，再执行 frontend check/build、docs build、Rust fmt/clippy/test、Go/Java/包测试与安全审计。不得安装系统包或绕过失败门禁。
9. 输出可审阅提交与验证报告后，按 APPROVAL-001 对具体 GitHub 推送目标请求确认。

## 后续进展补记（以 STATUS.md 为当前状态）

- 原有运行进程已完成；不存在待继续的构建进程。
- Go 实际21个模块全部tidy并构建通过，16个 main包需显式GIT_DIR/GIT_WORK_TREE纠正Go工作树VCS检测；不禁用stamping。
- 根工作区和 docs npm audit 最终均零告警；根目录source-map-resolve补丁+decode-uri-component0.5.1已实测。
- 文档站latest依赖 next16.4.3 / TypeScript7 / Fumadocs16.17.9 / MDX15.4.10 / Lucide1.64 / ZBSearch4.0.1构建与postbuild及10项SEO测试全部通过；42篇文档检查通过，真实中文查询通过。
- 3887文件format通过，lint exit0但仍有警告；完整前端check已观察失败，Vue compiler / vue-tsc 不支持TS7旧API，HappyDOM/CodeMirror亦有报错。
- GitHub脚本314测试全部通过；源码scripts旧Win7 sccache契约仍未处理。
- Shared GithubIcon新增于docs/components/GithubIcon.tsx，导航/侧栏/贡献者页复用。新增README.pl-PL和homepage复活的社区渠道已清除。
- 尚未升级Java/Gradle；registry metadata排序需使用release，旧snapshot最后一个版本的推断不可靠，不可据此盲改。
- 用户两项异步策略问题仍未答复，不能假定允许GTK/TS兼容版本例外。Rust解析仍被latest GTK与latest Tauri的原生链接冲突阻挡。
- 分支未形成提交，未推送、未部署。继续从STATUS.md的具体待办推进，不要重做已通过的验证，除非新改动影响其结果。

## 2026-10-30 继续推进：用户仅做最终验收（当前阶段）

用户明确要求“那就继续！而不是停下！我只做最终验收！中间完全由你负责！”。兼容性、迁移、修复的中间决策已委托给 Agent，不能再次为 GTK/TS 等细节停下询问。优先最新稳定版，有可复现 API/ABI/旧协议约束时保留维护兼容线并列明。没有授权部署其他主机/服务、外部消息、改凭据或发布 npm。

### 上游与主要成果

- 已显式 fetch 今天上游 `6bac6034b5b3e17780d49b80363db026b6a7003e` 至 `refs/remotes/upstream-sync/20261030`；153 文件增量从 1b8dfd96 干净 `git apply --3way --index` 应用。HEAD 仍1dfead3c，尚未形成提交/推送。
- 根 TS 改用微软维护的兼容包 `npm:@typescript/typescript6@6.0.2`，docs 独立保留 TS7；GTK0.18.2。
- 桌面前端 Vite 生产构建通过：`frontend-build-ts6.log`。最新类型检查在16GiB heap下通过：`frontend-typecheck-16g.log`（此前默认4GiB OOM已诊断，不能当代码失败重复跑）。
- 完整前端 29953 passed /16 failed /2 pending（`frontend-full-latest.json`）。比最初121失败显著收敛；六个编辑器/预加载/品牌文件102针对性测试已经全部通过。
- 真正修复：错误码validator兼容DBX和旧PanDB两个协议前缀、URL参数前导?往返、分页短页已知总数不完整判定。恢复PanDB工具栏名称/URL和图标alt；部分旧测试按fork身份适配，不能全局改内部协议/CLI名称。
- HappyDOM21.0.4同步selectionchange违反浏览器排队/合并事件契约，造成CodeMirror嵌套更新。已用 `patches/happy-dom@21.0.4.patch` 修复并保留最新版本；新独立测试 `selectionChangeScheduling.test.ts` 红到绿通过。没有关闭编辑器保护/忽略错误。
- Dayjs1.11.23 timezone插件 `v-=v%1e3` 对负毫秒向零截断导致 -1ns 显示 -00:01 而非UTC。新增 `patches/dayjs@1.11.23.patch` 改为floor秒边界；已修复无行末换行的patch格式，安装通过。原始不合法patch尝试已停止并改正确Git patch，没有绕过包校验。
- 运行中的 `frontend-remaining-final.json/log` 是最后8个文件的针对性验证；待看结果。剩余16项大部分是旧上游品牌/镜像/配置key预期与PanDB不一致，另1项即dayjs负时间。
- metadata.ts当前SITE_NAME/description改PanDB，OG图片本地/logo.png512x512（实际PNG已查看，与现有金色太阳标志一致）；`SITE_URL`仍旧默认dbxio但支持NEXT_PUBLIC_SITE_URL，未部署任何网站。后续生产域名不在本轮已有授权中，不可编造已上线。

### Rust

- `cargo +1.99.1 check -p dbx-core --no-default-features --locked` 已通过：`cargo-check-core-final-pass.log`，仍有17warnings需后续整理。
- 已迁移SQLparser0.63 OrderBySort（4处）、Reqwest0.13 query/form flags、SHA2新hybrid-array逐字节hex（与已有update_portable相同方式）、ZIP9文件名Result、Russh0.65 PublicKeyOrCertificate到原有host-key verifier（不绕过known-hosts）、Rusqlite0.41迭代器/签名/整数检查、AES/GCM0.11/CBC0.2新API、安全OS随机数getrandom、Sysinfo0.40 Result API、Minijinja3上下文、QuickXML0.42 UTF8名/属性/独立GeneralRef与CDATA等。
- `SqlU64`位于storage.rs文件末尾，显式保存旧unsigned↔signed SQLite INTEGER检查，不做截断。定时备份COUNT用i64，不改变100任务上限。
- MySQL0.37.1缺少源码依赖的`deprecate_eof`扩展，恢复现有固定Git fork0.37.0（并未比原主线降级），见COMPATIBILITY.md。PG fork HEAD115f9fef已只读核验但尚未替换原ac8固定rev。
- `prepare-vendor-ports.py`在/tmp校验registry tar SHA并三方移植。Wry0.55.1已移植到0.57.1，root只copy src/Cargo/许可/补丁说明，未复制测试私钥或上游CI；保留WebView2进程恢复/剪贴板/浏览器目录，采用新版GTK调用和Cocoa文件URL处理。修好Cocoa parent缺闭括号后rustfmt解析通过。详细触碰98 runtime文件见vendor-runtime-touch-list.json。
- rumqttc0.24已移植到0.25.1，但新Cargo默认use-rustls触发原有“旧Rustls后端已退役”compile_error。不能删除保护以过测试。现已把vendor Cargo默认改为use-native-tls，需重新generate-lockfile，再check CLI/MCP/Web。上一次ported检查日志仍是旧失败结果，勿当新结果。
- windows-core直依赖已对齐windows0.62.2，避免误用新0.100 ABI。
- 尚未完成Tauri desktop默认features编译。Tiberius0.13缺少vendor send_attention，updater2.14缺少自有restore_update；prepared候选tiberius移植约千行冲突，不可盲选。可保留带实际功能/安全修复的已维护fork并明确例外，或完成回移，但必须验证ATTENTION/UTF16和签名/安装安全边界。
- Wry0.57与WebView2 sys0.39需要对齐。当前sys vendor仍0.38.2（另有registry0.39.1），Win7 loader hash保护须保留；pageant0.2.5、ctor1.x迁移待核。不要通过绕过校验解决。

### 安全审计

`rust-osv-findings.json`：查询992个registry锁定crate，14个包有记录（包含不可达/仅构建/维护警告，尚未分类）：block-buffer0.10.4,glib0.18.5,h2.3.27,lru.16.4,proc-macro-error1.0.4,rsa0.10-rc18,rustls.21.12,rustls-pemfile2.2,webpki.101/.102,unsafe-libyaml.2.11,zbus_names4.3.5,zvariant5.15.1,zvariant_derive5.15.1。须查官方OSV/RustSec详情、cargo tree定位、升级或回移安全修复；不能声称Rust零漏洞，不能简单ignore。RSA可能无已修复版本且只私钥解密路径易受攻击，需要查真实可达性，而不是盲删SSH兼容。

### Java/Gradle

- 临时目录toolchains下已SHA验证JDK27.0.1.0.2+1、JDK21.0.13+9、Gradle9.8.2；不装系统包、不改默认。
- Java21是现有agent的target与JRE。Gradle由JDK21运行稳定；JDK27首次60s探针超时，诊断help在21成功12s。不要再重跑无日志的27探针。
- 已升级15份Gradle清单（java-upgrade-files.json），保留6项故意旧兼容：H2旧格式矩阵、MongoDB Legacy3.12、Intersystems/GBase特殊发布线。Mavenmetadata release也会指向旧2018或_nobson变体，不能盲目按它降级功能。
- Kafka4.4移除alterConfigs，现有旧broker fallback需要它；保留最新维护3.9.2，写入COMPATIBILITY。无新功能删减。
- java-compile-lts.log第一轮唯一Kafka错误；修正后 `java-compile-kafka-compat.log`仍在继续（session56142），已到spanner/snowflake，需看最终结果。仅编译源码/测试源码，不执行可能连真实数据库的integration test。

### 运行会话 / 下一步

- 65769：最后8个前端文件回归（读取frontend-remaining-final.json）。
- 56142：Java compileJava compileTestJava（超时600s，TTY可Ctrl-C）。
- 47850：旧CLI/MCP/Web check已因rumqttc旧默认Rustls保护失败，不应复用。
- 源码/产物日志均在/tmp/pandb-upgrade-20261029。状态见实际日志，不能把进程存在当成功。
- 要做：重新锁定native-TLS rumqttc并编译CLI/MCP/Web；Rust lib tests需独立DBX_DATA_DIR/PANDB_DATA_DIR任务目录避免碰用户真实数据；处理审计；Tauri桌面构建；Go单元/Java安全单元与插件SDK；所有workflow防止复活自动外发；所有品牌和下载来源指向本fork而非静默回跳上游；最后全量格式/类型/测试/构建/审计、真实本地Web+MCP+SQLite流、审阅和GitHub更新。不要再向用户询问中间细节。

## 2026-10-30 继续：运行证据与远程验证阶段

- `/tmp/pandb-upgrade-20261029` 已因环境恢复消失。后续工具及证据改存 `/home/arch-gpt--notebook001/work/pandb-upgrade-20261030`。不得将旧 session ID 或已消失日志当作当前验收证据。
- 完整 frontend run-check 曾通过全部5项；最后品牌和Node26更改后再次运行。GitHub脚本314项本轮重新通过。
- DBX协议producer及Rust预期已修回canonical DBX；前端仍兼容历史PanDB入站。新版ZIP9额外测试fixture getter已迁移。
- 默认桌面完整link、workspace nextest运行中。内置transwarp YAML缺少max_length已补128，并新增所有37份内置方言逐个加载回归。
- 实际Chrome登录/连接/CodeMirror输入SQL/执行按钮/Canvas grid视觉验收曾通过，中文与1.25数据正确。MCP经Web读取通过；显式保存readOnly=true后INSERT被拒。旧legacy未配置策略可写，不能声称默认只读通过。
- 新SQLite3.54.0实际读取3.51.3旧加密fixture、错误key拒绝、只读SHA不变均通过；原fixture位于已消失tmp，需从构建输出重建持久验证证据。
- Windows updater ZIP4已迁到ZIP9，旧TLS、lru问题栈移除；GLib官方修复已回移。RSA无fixed及维护警告仍需分类台账，不能声称全Rust0告警。
- Node26.13.0是2026-10-28官方当前LTS，已验证下载SHA；.nvmrc/CI/Docker/Nix统一。pnpm12原生Nix包已补nodejs-slim passthrough，Git source hashes按当前Cargo.lock重算；需GitHub Nix runner实测，不可称已通过。
- 新maintenance-verification仅维护分支push和手动运行，contents:read，Windows check与Nix验证；无部署/发布/外部消息。准备维护分支提交后运行；主分支仍未更新。
- origin/main多一份contributors更新已读回并保留，禁止force push。

## 2026-10-30 持久验证结果与修复收口

- 维护分支已推送，首次GitHub run 41025526627：Nix成功，Windows定位到Wry once_cell移除后自有回调未迁移；已改std::sync::LazyLock并在本机Windows target独立cargo check通过。Win32 clipboard的Rust2024 unsafe操作已显式封装，保留所有数据和窗口校验。
- Nix runner实测pnpm12原生包及依赖fetch成功，artifact已取回并应用验证哈希 sha256-9HEZQTnW4mqLoZJPRn7Vh2GJ3UkD7ySrbBWU8aqLASg=。
- Node26前端完整check5项通过，生产build通过；docs Node26 build+postbuild通过，314 GitHub脚本再次通过，工作流无重复YAML键。
- 插件CLI23项和packager测试均通过；独立packager旧ZIP4 lock已刷新到ZIP9。
- Go全module OSV查询发现unused graph及实际驱动依赖遗留；13个受影响模块已处理，其中Argo/Hive广泛更新下载超时后改为精确模块更新成功。iotdb bench原先未充分升级，本轮精确更新与unit通过。Go x/crypto0.58仍是官方latest，其GO-2026-5932针对未维护openpgp；生产依赖扫描仅使用md4/pbkdf2，没有openpgp，不得把module级告警直接当可达漏洞。仍需最终重审与21模块整体测试。
- Rust all-targets发现并修复SSH测试fixture新接口、MQTT Packet::read新接口、Sysinfo基准示例及Windows条件代码；SQL-schema测试遗漏tempfile已补既有当前版dev依赖。新Clippy要求as_chunks、expect_err等已按建议迁移，未关闭门禁。
- 对大型内部错误进行了Box封装并在Web/Tauri边界还原原错误，避免扩大异步栈；SecretStr清理改用现有依赖图zeroize1.9.1确保清理不被优化移除。需要等待当前Clippy与nextest真正完成后才能报告通过。
- 默认桌面完整cargo build已本轮观察exit0（4m54s），后续core变更需最终再check/link。
- SQLite旧加密fixture已重建到work/pandb-upgrade-20261030/sqlite-compat：旧3.51.3生成、新3.54.0只读验证、wrongkey拒绝、SHA不变均exit0，有result.json持久证据。
- 未更新main、未部署或发布安装包。新增upstream缓存清理schedule与自动issue-label同步在fork改为显式手动运行，避免同步代码引入未授权外部写入。已有contributors更新被保留。

## 2026-10-30 最终验证阶段（持久日志以实际结果为准）

- 第一轮完整nextest：11526执行、11507通过、19失败、370原有跳过。根因已分别修正：JWT11明确AWS-LC功能；MQTT库显式选provider；agent依赖registry恢复既有上游地址；Linux窗口标题保持PanDB；SQLParser新版诊断字串兼容且保留结构重解析；ZIP9更早拒绝伪造尺寸的回归验证；SSE测试服务完整读取HTTP请求。
- 第二轮同范围nextest（采用line-tables-only调试信息、8MiB栈）：11526全部通过，370跳过，日志rust-workspace-runtime-fixes.log。SQL-core713与原始SQL分析46个集成case独立通过；方言173通过。
- 严格CI全target Clippy新代码已exit0（rust-clippy-runtime-final.log）；默认桌面完整build exit0（desktop-runtime-final-build.log，7m46s）。
- 真正DuckDB native2.0.0 stdio协议、BIGINT/中文/DECIMAL、Parquet读写、checkpoint与重开全部通过，证据duckdb-acceptance/result.json。DuckDB独立all-target check和TDengine当前check都通过。
- Vendored组件单独OSV复查发现Pageant0.2.1实际缺CVE-2026-102820长度检查；已补官方边界修复，并修复同处SID/安全属性指针生命周期和handle/view错误处理；Windows target check与-Dwarnings Clippy通过。维护CI新增真实Windows内核mapping release回归。
- SQLite3.54.0存在已公布的32位缺陷；保持MC稳定2.6.1，拷贝官方维护提交30f1的3.54.1内核与头文件（官方SHA3一致），稳定tag生成器重生成，全部codec hook计数不变。七种3.51.3旧密码文件读取、wrong-key拒绝、SHA不变再次全过。现在vendor版本1.540100.0；还需最后默认build/跨平台收口。
- GitHub run41029991835（31fb3836）：Windows、macOS、Nix全成功；最后native安全补丁会再推维护分支验证。main未更新，已读回其protected=false，SHA仍b74f8b14。
- DOCX完整ZIP/逐段文本复查0社区词/链接残留，无媒体部件；输出绝对路径仍为Downloads/PanDB_已删除社区相关内容.docx。
