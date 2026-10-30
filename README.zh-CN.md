<div align="center">

<img src="brand/pandb-logo.svg" alt="PanDB 标志" width="144" />

# PanDB

**轻量、AI 原生的数据库管理平台，支持 70+ 种数据库，提供桌面端、Docker、CLI 与 MCP Server。**

[GitHub](https://github.com/ActiveInAI/PanDB) · [English](README.md)

</div>

## 为什么选择 PanDB？

<table>
  <tr>
    <td width="50%">
      <h3>🪶 25 MB，极致轻量</h3>
      <p>无需 Java 运行环境，无需 Python 虚拟环境，不内嵌 Chromium。PanDB 是单个小巧的二进制文件——下载、安装、连接。DBeaver 依赖 Java；TablePlus 是 Freemium。PanDB 全平台可用，无需额外运行时。</p>
    </td>
    <td width="50%">
      <h3>🤖 AI 原生集成在编辑器里</h3>
      <p>选中一张表，描述你想要什么，直接得到 SQL——无需在工具之间复制粘贴。支持 Claude、OpenAI，或通过 Ollama 使用本地模型。内置安全检查会在执行前审查 AI 生成的 SQL。</p>
    </td>
  </tr>
  <tr>
    <td>
      <h3>🔌 MCP 协议：你的数据库，AI 就绪</h3>
      <p>PanDB 原生支持 Model Context Protocol。Claude Code、Cursor、Windsurf 等 AI 编程助手可以直接通过你已配置的数据库连接查询数据。一次配置，处处可用。</p>
    </td>
    <td>
      <h3>🌐 桌面端 + Docker + Web</h3>
      <p>macOS、Windows、Linux 原生应用。通过 Docker 自托管供团队访问。Web 版本适配纯浏览器环境。同样的功能，同样的连接配置。</p>
    </td>
  </tr>
  <tr>
    <td>
      <h3>📨 不止数据库</h3>
      <p>消息队列与中间件同样是一等公民：Kafka、RocketMQ、RabbitMQ、Pulsar、MQTT 控制台，外加 Nacos、Consul、ZooKeeper、etcd。在数据库旁边直接查看主题与消息，无需再开一个工具。</p>
    </td>
    <td>
      <h3>🧩 插件生态</h3>
      <p>从内置商店安装经过签名校验的沙箱插件扩展 DBX——S3、Kubernetes、LDAP 等。也可以用 Go / TypeScript SDK 开发自己的插件。</p>
    </td>
  </tr>
</table>

## 功能特性

### 70+ 种数据库，一个工具搞定

MySQL、PostgreSQL、SQLite、Cloudflare D1、Redis、MongoDB、DuckDB、ClickHouse、SQL Server、Oracle、Elasticsearch、Easysearch、MariaDB、TiDB、OceanBase、openGauss、GaussDB、KWDB、KingBase、Vastbase、GoldenDB、Doris、SelectDB、StarRocks、Manticore Search、Redshift、DM、TDengine、虚谷 XuguDB、CockroachDB、Access、HighGo、UXDB 等数据库都能直接连接。Agent/JDBC 方向的配置还可扩展到 H2、Snowflake、Trino、Hive、DB2、Informix、Neo4j、Cassandra、BigQuery、Kylin、SunDB、JDBCX 和自定义 JDBC。新增的原生与 Agent 驱动还覆盖了 Databricks、SAP HANA、Teradata、Vertica、Firebird、Exasol、崖山 YashanDB、GBase、Databend、RQLite、Turso、InfluxDB、QuestDB、IoTDB、etcd、IRIS 等。全部装进约 20 MB 的应用里，不内嵌 Chromium。

### 查询编辑器

CodeMirror 6 语法高亮、元数据感知自动补全、`Cmd+Enter` 执行、选中 SQL 执行、SQL 格式化、诊断提示，9 种编辑器主题。查询历史、常用 SQL 片段、标签页恢复和 SQL 文件执行让重复工作更顺手。

### AI SQL 助手

用自然语言描述你的需求，直接生成 SQL。还能解释查询、优化 SQL、修复错误，并通过内置安全检查执行 AI 生成的 SQL。支持 Claude、OpenAI、本地模型或任何 OpenAI 兼容端点。

### 数据表格

虚拟滚动，轻松应对大型结果集。行内编辑、保存前 SQL 预览、WHERE / ORDER BY 控件、DataGrip 风格过滤器、LIKE / NOT LIKE 右键过滤、排序、全文搜索、分页、列宽调整、自动列宽、行号、斑马纹和完整单元格详情。支持导出或复制为 CSV、JSON、Markdown、XLSX、INSERT 语句。

### Schema 工具

- **结构浏览** — 数据库、Schema、表、字段、索引、外键、触发器，支持侧边栏搜索和置顶
- **对象浏览** — 按类型分组查看过程、函数、视图，并在支持的数据库中编辑源码
- **表结构编辑器** — 对支持的数据库执行可审查的字段和索引变更
- **ER 关系图** — 可视化表间关联
- **Schema 对比** — 跨连接对比表结构差异
- **执行计划** — 可视化查询执行计划
- **字段血缘** — 字段级血缘分析
- **数据库搜索** — 在大型 Schema 中快速查找对象

### 数据操作

- **数据导入** — CSV、Excel
- **数据迁移** — 在数据库之间迁移数据
- **数据库导出** — 完整数据库导出
- **数据对比** — 对比表数据并审查同步结果
- **SQL 文件执行** — 直接执行 `.sql` 文件
- **文件预览** — 拖入 Parquet、CSV、JSON 即时预览（基于 DuckDB）
- **连接导入** — 从 DBeaver 或 Navicat 导入连接配置

### 专项浏览器

- **Redis** — 模式匹配搜索、批量键操作、命令执行器、TTL 编辑，全数据类型支持（String、Hash、List、Set、ZSet、Stream）
- **MongoDB** — 文档增删改查、分页浏览，支持 Atlas 和副本集 URL 直连

### 消息队列与中间件控制台

- **Kafka / RocketMQ / RabbitMQ / Pulsar** — 主题、消费组、消息浏览 / 查询 / 追踪、Broker 监控、权限与策略
- **MQTT** — 主题树导航、订阅与发布
- **Nacos / Consul / ZooKeeper / etcd** — 服务发现、KV / 配置浏览、健康状态与 ACL

### 插件系统

- **为扩展而生** — 新的连接类型与工具以插件形式接入：S3 浏览、Kubernetes、LDAP 等，内置商店一键安装
- **签名与沙箱** — 插件包安装前签名校验，插件 UI 运行在沙箱中并拥有独立 sidecar 进程
- **开发你的插件** — Go / TypeScript SDK，`npx @dbx-app/plugin-cli` 一行起步，通过 [`t8y2/dbx-store`](https://github.com/t8y2/dbx-store) 发布到商店

### 安全与连接

SSH 隧道（密钥和密码认证）· 数据库和 AI 代理设置 · 断线自动重连 · 危险操作确认对话框 · 加密导出/导入连接配置 · 连接颜色标记 · 驱动商店与可选 JDBC 插件

### 精致 UI

深色模式原生标题栏同步 · 9 种编辑器主题 · English、简体中文、Español · 布局偏好设置 · 内置自动更新

## AI 编程助手集成 (MCP)

PanDB 提供 [MCP Server](packages/mcp-server/)，让 AI 编程助手直接使用 PanDB 中已配置的数据库连接查询数据。

```bash
npx @dbx-app/mcp-server
```

在 `.mcp.json` 中添加：

```json
{
  "mcpServers": {
    "dbx": { "command": "npx", "args": ["-y", "@dbx-app/mcp-server"] }
  }
}
```

连接 allowlist 和“只读 / 数据读写 / 完全访问”三档执行权限统一在 PanDB 的“设置 → MCP”中管理。机器可读值仍为 `read_only`、`safe_write`、`high_risk_write`；客户端配置无需声明权限或连接范围环境变量。

为兼容升级，旧配置中的 `DBX_MCP_ALLOW_WRITES=0`（或 `false`）仅在中央 MCP 策略首次保存前继续作为只读限制；它不能开启写入，也不能覆盖已经保存的中央策略。

Windows 便携版需要在 MCP 配置中设置 `PANDB_DATA_DIR`，指向 `PanDB.exe` 同级的 `data` 目录（即包含 `dbx.db` 的文件夹）。

如果连接的是 PanDB Web 或 Docker 部署，请让 MCP Server 指向 Web 后端 API。如果 Web 登录页需要密码，`PANDB_WEB_PASSWORD` 填写同一个 Web 登录密码：

```json
{
  "mcpServers": {
    "pandb": {
      "command": "pandb-mcp-server",
      "env": {
        "PANDB_WEB_URL": "http://localhost:4224",
        "PANDB_WEB_PASSWORD": "你的 Web 登录密码"
      }
    }
  }
}
```

支持 Claude Code、Cursor、Windsurf 等 MCP 兼容的 AI 助手。可列出连接、浏览表、执行 SQL，还能直接在 PanDB 界面中打开表。

## 安装

PanDB 首个公开发行版正在准备中；当前请从源码构建，并按下方“快速开始”完成开发环境配置。已有安装在迁移期间保持兼容，但旧下载渠道不代表 PanDB 发行版。

## 自托管 (Docker)

在当前源码目录构建并运行 Web 版：

```bash
docker compose -f deploy/docker-compose.yml up --build
```

容器就绪后访问 `http://localhost:4224`。如需通过 nginx 等反向代理发布到 `/pandb` 这类子路径，使用新的运行时变量：

```yaml
environment:
  - PANDB_PUBLIC_BASE_PATH=/pandb
```

如果自行从源码构建前端并希望使用绝对资源路径，可在 `pnpm build` 前设置 `VITE_PANDB_BASE_PATH=/pandb/`。已有部署仍支持旧的 `DBX_*` 变量。

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) >= 1.88

#### 系统依赖

**macOS：**

无需额外安装。

**Linux (Ubuntu/Debian)：**

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf libssl-dev
```

**NIXOS/NIX :** 

<a href="README-NIX.md">查看 README-NIX.md</a>

**Windows：**

无需额外安装。

### 开发

```bash
make
```

`make` 会在需要时安装根目录依赖，并启动本地 Tauri 桌面端开发环境。

开发版可与已安装的 PanDB 同时运行，并共享本地连接和历史数据。请避免在两个窗口中同时修改同一个连接或全局设置。

> [!TIP]
> DuckDB 从源码编译较慢。如果不涉及 DuckDB 功能，可以跳过以加速本地构建：
>
> ```bash
> # 快速检查（跳过 DuckDB）
> make cargo-check-fast
> make cargo-test-fast
>
> # Tauri 开发模式跳过 DuckDB
> make dev-fast
> ```
>
> `--no-default-features` 仅影响本地开发，发布构建（`pnpm tauri build`）始终包含 DuckDB。

Web 版本：

```bash
make dev-web       # 前端
make dev-backend   # 后端
```

文档站：

```bash
make docs
```

PanDB 官网文档位于 `docs/` 目录。如果你想贡献官网内容或文档页面，请修改 `docs/` 下的文件，并运行 `make docs` 在本地预览文档站。

需要干净、可重复创建的本地数据库实例时，可使用 [`deploy/database/`](deploy/database/README.zh-CN.md) 下的带版本 Docker Compose 配方：

```bash
make db-list
make db-verify DB=mysql@8.4
```

JDBC Agent 驱动开发工程位于 `agents/` 目录：

```bash
cd agents
./gradlew test
```

本地驱动安装流程会优先查找 `agents/drivers/<db-type>/build/libs/` 下的构建产物。

### 构建

```bash
make package
```

安装包输出在 `src-tauri/target/release/bundle/` 目录。

`pnpm build` 会在运行 Vite 前显式清理根目录的 `dist/`，确保桌面打包使用重新生成的前端资源。

## 技术栈

| 层级   | 技术                                                                                                                                                                                                             |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 框架   | [Tauri 2](https://tauri.app/)                                                                                                                                                                                    |
| 前端   | [Vue 3](https://vuejs.org/) + TypeScript                                                                                                                                                                         |
| UI     | [shadcn-vue](https://www.shadcn-vue.com/) + Tailwind CSS                                                                                                                                                         |
| 编辑器 | [CodeMirror 6](https://codemirror.net/)                                                                                                                                                                          |
| 后端   | Rust + [sqlx](https://github.com/launchbadge/sqlx) / [tiberius](https://github.com/prisma/tiberius) / [redis-rs](https://github.com/redis-rs/redis-rs) / [mongodb](https://github.com/mongodb/mongo-rust-driver) |

## 文档

- [文档源码](https://github.com/ActiveInAI/PanDB/tree/main/docs) — 功能说明与使用教程
- [数据库测试实验室](https://github.com/ActiveInAI/PanDB/tree/main/deploy/database) — 用于开发和验证的本地数据库配方
- [贡献指南](CONTRIBUTING.zh-CN.md) — 如何认领 Issue 并提交 PR
- [Web API 参考](docs/content/docs/web-api.cn.mdx) — Docker/Web 部署的 HTTP API
- [示例代码](examples/) — CLI、MCP、Docker 与 API 示例

## 常见问题

<details>
<summary><strong>PanDB 是免费的吗？</strong></summary>
是的。PanDB 基于 Apache-2.0 协议开源，所有功能均免费使用。
</details>

<details>
<summary><strong>PanDB 会收集用户数据吗？</strong></summary>
不会。PanDB 不收集任何遥测数据。开启更新通知时，桌面端会通过所选更新源检查新版本并静默下载安装包；下载并校验完成后，更新入口显示提示，只有点击“重启并更新”才会安装。已下载的安装包会保留到下次启动，也可忽略该版本。你可以在设置中关闭自动检查和下载。
</details>

<details>
<summary><strong>可以离线使用吗？</strong></summary>

可以。桌面端完全支持离线使用。内网环境安装驱动时，可在有网机器获取所需 JDBC 制品，传输到内网机器后，在 PanDB 的「设置 > 驱动管理」中导入。AI 功能需要网络访问模型端点（或通过 Ollama 使用本地模型）。
</details>

<details>
<summary><strong>PanDB 和 DBeaver / TablePlus / Beekeeper Studio 有什么区别？</strong></summary>
PanDB 仅 20 MB，无需运行时依赖（无需 Java、无需 Python）。AI 和 MCP 是原生内置功能，不是插件。单一代码库同时支持 70+ 数据库、桌面端、Docker 和 Web。
</details>

<details>
<summary><strong>支持哪些数据库？</strong></summary>
MySQL、PostgreSQL、SQLite、Cloudflare D1、Redis、MongoDB、DuckDB、ClickHouse、SQL Server、Oracle、Elasticsearch、Easysearch、Qdrant、Milvus、Weaviate、MariaDB、TiDB、OceanBase、openGauss、GaussDB、KWDB、KingBase、Vastbase、GoldenDB、Doris、SelectDB、StarRocks、Manticore Search、Redshift、DM、TDengine、虚谷 XuguDB、CockroachDB、Access、HighGo、UXDB 等。JDBC 方向配置可扩展到 H2、Snowflake、Trino、PrestoSQL、Hive、DB2、Informix、Neo4j、Cassandra、BigQuery、Kylin、SunDB、JDBCX、Databricks、SAP HANA、Teradata、Vertica、Firebird、Exasol、崖山 YashanDB、GBase 8a/8s、Databend、RQLite、Turso、InfluxDB、QuestDB、IoTDB、etcd、ZooKeeper、Nacos、IRIS 及自定义 JDBC 连接，并支持消息队列管理（Pulsar、Kafka、RocketMQ）。
</details>

<details>
<summary><strong>如何报告 Bug 或请求新功能？</strong></summary>
在 <a href="https://github.com/ActiveInAI/PanDB/issues">GitHub Issues</a> 提交 Issue。
</details>

## 贡献者

<a href="https://github.com/ActiveInAI/PanDB/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ActiveInAI/PanDB&max=300&columns=15" />
</a>

## Star History

<a href="https://www.star-history.com/?repos=ActiveInAI%2FPanDB&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&legend=top-left" />
 </picture>
</a>

## 开源协议

[Apache-2.0](LICENSE)
