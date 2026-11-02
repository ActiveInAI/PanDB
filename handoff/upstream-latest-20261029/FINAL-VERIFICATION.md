# PanDB 同步与依赖升级验收（2026-11-02）

已验证代码提交：`afe3de9b41923439e34508458d23cc07726e84d0`，目标为 `ActiveInAI/PanDB/main`。本轮冻结上游基线为 `6bac6034b5b3e17780d49b80363db026b6a7003e`。Windows支持目标收敛到Windows11+，停用Win7自动构建与发布。

[主分支CI41381463333](https://github.com/ActiveInAI/PanDB/actions/runs/41381463333)已完成：**50项成功、0失败、2项按配置跳过**。跳过项为停用的Win7 bundle及旧JDBC汇总路径；Java测试、离线打包和仓库认证由独立任务验收。另有[维护验收CI41380834793](https://github.com/ActiveInAI/PanDB/actions/runs/41380834793)五项全部成功。

| 验证 | 观察结果 |
| --- | --- |
| Rust工作区 | 本轮完整功能nextest **11564通过、0失败、375按既有配置跳过**；all-targets、`-D warnings`的严格Clippy成功 |
| Windows / macOS / Nix | 当前代码五项维护验收全成功，含真实ZooKeeper3.5.5与SASL3.7.0；Nix实际重建JS依赖及Rust源vendor并验证固定哈希 |
| Windows DuckDB | 实际native v2.0.0：引擎版本、SQL中文/BIGINT/DECIMAL、Parquet写读、checkpoint与重开共4项成功；PE依赖检查通过 |
| Windows Pageant | 当前0.2.5：目标检查、严格Clippy、本机账户名8项及真实Windows release13项成功 |
| 前端 | 本机Node26下连接类型/format/lint/typecheck/全量test五项成功，生产build成功；本轮主CI两分片及类型检查成功 |
| 文档 | 修改反馈入口与移除社区资源后build/postbuild通过；278索引页、282本地化页、1546导出文件，导出中无已删除的4张社区图片 |
| 脚本 / 发布流程 | 本机327项GitHub脚本加13项发布脚本，合计340通过；未就绪ZooKeeper阻止业务测试、延迟就绪及失败日志/清理回归成功 |
| DuckDB组件 | 本机70项全通过；按生产路径修正的注册取消用例连续20轮成功；本轮CI组件70项全成功 |
| 包分发演练 | 主CI完成CLI/MCP/插件包测试及publish dry-run；未执行安装包或外部分发发布 |
| Go / Java / 插件 | 本机基线Go1.27.3下21模块build/short unit通过、Java持久JUnit527通过、插件宿主58及Rust CLI23/packager8通过；相关本轮主CI全部成功 |
| SQL / 方言基线 | SQL-core713、原SQL分析集成46、方言173通过，37份内置catalog可加载；本轮工作区重新验收成功 |
| Linux桌面 / 旧数据基线 | 完整默认功能native build/link成功；实际SQLite3.54.1库读取七种3.51.3旧加密格式、拒绝wrong-key且只读SHA不变，Argon2验证旧PHC成功 |
| 界面 / HTTP / MCP基线 | 真实浏览器登录、连接、编辑执行SQL与Canvas中文结果核对通过；HTTP8项、MCP4项，含显式只读拒写；本轮未更改这些业务实现 |

社区清理覆盖当前文档文字/关系、仓库入口、QQ群发布通知、14份未用翻译及4个未引用的二维码/图标。GitHub反馈页使用仓库已启用的Issues入口；上游专用Homebrew/Scoop/R2发布根作业限定在上游仓库执行，未替用户猜测其它发布目标。

构建修复保留实际编译与运行检查。cc-rs会从RUSTC_WRAPPER隐式继承sccache，Windows C++因此改用按锁文件、工具链和MSVC/SDK环境区分的产物缓存。同一Windows驱动Cargo release构建首次为18分46秒，本轮缓存复用为44.52秒；这是构建耗时，不能当作应用运行性能提升。

根工作区未使用的旧Stylus/glob/minimatch可选链已清除，安全版本约束在隔离工程中验证兼容，应用未新增必需编译器依赖。ICU时区测试显式准备必需扩展；ZooKeeper就绪在TCP之外检查真实服务状态，保留所有业务断言及版本矩阵。Nix的JS依赖哈希由真实构建报告取得并重新验证，未猜测或绕过完整性校验。

当前三套npm审计均为0，GitHub开放依赖告警实读为0，未执行dismiss。**不能宣称每个依赖都采用注册表最新大版本，也不能宣称全项目零漏洞。** API/ABI及旧格式兼容线见[COMPATIBILITY.md](COMPATIBILITY.md)；RSA上游无补丁和其它维护状态见[SECURITY-REVIEW.md](SECURITY-REVIEW.md)。未添加审计忽略项。

真实供应商环境、实际用户数据库和用户业务验收不由源码/CI结果替代；本次未迁移真实数据库、未更改服务或凭据、未发布安装包。DOCX交付文件为 `PanDB_已删除社区相关内容.docx`，已重新验证ZIP/XML完整、社区文字/链接0、嵌入图片0。结构化结果见[verification-summary.json](verification-summary.json)，原始日志和本地交付回执保存在本任务受控证据目录。
