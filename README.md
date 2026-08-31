<div align="center">

<img src="brand/pandb-logo.svg" alt="PanDB logo" width="144" />

# PanDB

**Lightweight, AI-native database management for 70+ databases. Desktop, Docker, CLI, and MCP Server.**

[GitHub](https://github.com/ActiveInAI/PanDB) · [简体中文](README.zh-CN.md)

</div>

## Why PanDB?

<table>
  <tr>
    <td width="50%">
      <h3>🪶 20 MB, zero runtime bloat</h3>
      <p>No Java JRE. No Python venv. No bundled Chromium. PanDB ships as a single small binary — download, install, connect. DBeaver needs Java; TablePlus is macOS-only. PanDB runs everywhere with nothing extra.</p>
    </td>
    <td width="50%">
      <h3>🤖 AI that lives in your editor</h3>
      <p>Highlight a table, describe what you want, get SQL back — no copy-paste between tools. Works with Claude, OpenAI, or local models via Ollama. Built-in safety checks review AI-generated SQL before it runs.</p>
    </td>
  </tr>
  <tr>
    <td>
      <h3>🔌 MCP: your databases, AI-ready</h3>
      <p>PanDB speaks the Model Context Protocol. Claude Code, Cursor, Windsurf, and other AI coding agents can query your databases through connections you already set up. One config, everywhere.</p>
    </td>
    <td>
      <h3>🌐 Desktop + Docker + Web</h3>
      <p>Native app on macOS, Windows, and Linux. Self-host via Docker for team access. Web version for browser-only environments. Same feature set. Same connections.</p>
    </td>
  </tr>
</table>

## Features

### 70+ Databases, One Tool

MySQL, PostgreSQL, SQLite, Cloudflare D1, Redis, MongoDB, DuckDB, ClickHouse, SQL Server, Oracle, Elasticsearch, Easysearch, Qdrant, Milvus, Weaviate, MariaDB, TiDB, OceanBase, openGauss, GaussDB, KWDB, KingBase, Vastbase, GoldenDB, Doris, SelectDB, StarRocks, Manticore Search, Redshift, DM, TDengine, XuguDB, CockroachDB, Access, HighGo, UXDB, and more. Agent/JDBC-oriented profiles extend PanDB to H2, Snowflake, Trino, PrestoSQL, Hive, DB2, Informix, Neo4j, Cassandra, BigQuery, Kylin, SunDB, JDBCX, and custom JDBC connections. New native and agent-driven drivers also cover Databricks, SAP HANA, Teradata, Vertica, Firebird, Exasol, YashanDB, GBase 8a/8s, Databend, RQLite, Turso, InfluxDB, QuestDB, IoTDB, etcd, ZooKeeper, Nacos, IRIS, and more. Message queue admin is also available for Pulsar, Kafka, and RocketMQ. All in a single ~20 MB app. No bundled Chromium.

### Query Editor

CodeMirror 6 with SQL syntax highlighting, metadata-aware autocomplete, `Cmd+Enter` execution, selected SQL execution, SQL formatting, diagnostics, and 9 editor themes. Persistent query history, saved SQL snippets, tab restore, and SQL file execution keep repeat work close at hand.

### AI SQL Assistant

Describe what you want in plain language — get SQL back. PanDB can explain queries, optimize SQL, fix errors, and run AI-generated SQL through built-in safety checks. Works with Claude, OpenAI, local models, or any OpenAI-compatible endpoint.

### Data Grid

Virtual-scrolled table that handles large result sets. Inline editing, SQL preview before save, WHERE / ORDER BY controls, DataGrip-style filters, LIKE / NOT LIKE context filters, sorting, full-text search, pagination, column resize, auto-fit, row numbers, zebra stripes, and full cell details. Export or copy as CSV, JSON, Markdown, XLSX, or INSERT statements.

### Schema Tools

- **Schema browser** — databases, schemas, tables, columns, indexes, foreign keys, triggers, with sidebar search & pin
- **Object browser** — grouped procedures, functions, views, and source editing where supported
- **Table structure editor** — reviewable column and index changes for supported engines
- **ER diagram** — visualize table relationships
- **Schema diff** — compare structures across connections
- **Explain plan** — visual query execution plan
- **Field lineage** — column-level lineage analysis
- **Database search** — find objects across large schemas

### Data Operations

- **Table import** — CSV, Excel
- **Data transfer** — migrate between databases
- **Database export** — full database dump
- **Data compare** — compare table data and review synchronization output
- **SQL file execution** — run `.sql` files directly
- **File preview** — drag & drop Parquet, CSV, JSON to preview instantly (powered by DuckDB)
- **Connection import** — bring connection profiles from DBeaver or Navicat

### Specialized Browsers

- **Redis** — key pattern search, batch key operations, command runner, TTL editing, and all data types (String, Hash, List, Set, ZSet, Stream)
- **MongoDB** — document CRUD with pagination, Atlas & replica set URL connection

### Safety & Connectivity

SSH tunnel (key & password) · database and AI proxy settings · auto-reconnect on connection loss · confirmation dialogs for destructive operations · encrypted config export/import · color-coded connections · driver store and optional JDBC plugin

### Polished UI

Dark mode with native title bar sync · 9 editor themes · English, 简体中文 & Español · layout preferences · built-in auto-update

## AI Agent Integration (MCP)

PanDB provides a separate [Rust-powered MCP server](packages/mcp-server/) that lets AI coding agents query databases using connections configured in PanDB. The MCP server is distributed independently from the desktop application, so installing PanDB does not automatically install the MCP executable.

```bash
npx @dbx-app/mcp-server
```

Add to your `.mcp.json`:

```json
{
  "mcpServers": {
    "dbx": { "command": "npx", "args": ["-y", "@dbx-app/mcp-server"] }
  }
}
```

Manage the connection allowlist and the **Read only**, **Data read/write**, and **Full access** modes in **PanDB Settings → MCP**. The machine-readable values remain `read_only`, `safe_write`, and `high_risk_write`; client configs do not need permission or connection-scope environment variables.

For upgrade compatibility, an existing `DBX_MCP_ALLOW_WRITES=0` (or `false`) remains a read-only restriction only until a central MCP policy is saved for the first time; it can never enable writes or override a saved policy.

Windows portable builds need `PANDB_DATA_DIR` in the MCP config, pointing to the `data` directory next to `PanDB.exe` (the folder that contains `dbx.db`).

For PanDB Web or Docker deployments, point the MCP server at the Web backend API. If the Web login page requires a password, set `PANDB_WEB_PASSWORD` to the same password used there:

```json
{
  "mcpServers": {
    "pandb": {
      "command": "pandb-mcp-server",
      "env": {
        "PANDB_WEB_URL": "http://localhost:4224",
        "PANDB_WEB_PASSWORD": "your-web-login-password"
      }
    }
  }
}
```

Works with Claude Code, Cursor, Windsurf, and any MCP-compatible agent. Supports listing connections, browsing tables, executing SQL, and opening tables directly in PanDB's UI.

## Installation

PanDB is currently distributed from source while its first public release is prepared. Clone this repository and follow the development instructions below. Existing installations remain compatible during the transition, but legacy download channels are not PanDB release channels.

## Self-Hosted (Docker)

Build and run the Web edition from this checkout:

```bash
docker compose -f deploy/docker-compose.yml up --build
```

Open `http://localhost:4224` after the containers are ready. To publish under a reverse-proxy context path such as `/pandb`, use the new runtime variables:

```yaml
environment:
  - PANDB_PUBLIC_BASE_PATH=/pandb
```

When building the frontend with an absolute asset base, set `VITE_PANDB_BASE_PATH=/pandb/` before `pnpm build`. The legacy `DBX_*` forms remain supported for existing deployments.

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) >= 1.88

#### System Dependencies

**macOS:**

No additional dependencies required.

**Linux (Ubuntu/Debian):**

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf libssl-dev
```

**NIXOS/NIX :** 

<a href="README-NIX.md">See README-NIX.md</a>

**Windows:**

No additional dependencies required.

### Development

```bash
make
```

`make` installs root dependencies when needed and starts the local Tauri desktop development environment.

Development builds can run alongside an installed PanDB instance and share its local data, including connections and history. Avoid changing the same connection or global setting in both windows at once.

> [!TIP]
> DuckDB compilation takes a while. If you're not working on DuckDB features,
> skip it to speed up local builds:
>
> ```bash
> # Fast checks (skip DuckDB)
> make cargo-check-fast
> make cargo-test-fast
>
> # Tauri dev without DuckDB
> make dev-fast
> ```
>
> The `--no-default-features` flag only affects local development.
> Release builds (`pnpm tauri build`) always include DuckDB.

Web version:

```bash
make dev-web       # frontend
make dev-backend   # backend
```

Documentation site:

```bash
make docs
```

The official PanDB documentation site lives in `docs/`. If you want to improve the website content or documentation pages, edit the files under `docs/` and run `make docs` to preview the site locally.

For clean, reproducible local database instances, use the versioned Docker Compose recipes under [`deploy/database/`](deploy/database/README.md):

```bash
make db-list
make db-verify DB=mysql@8.4
```

JDBC agent driver development projects live in `agents/`:

```bash
cd agents
./gradlew test
```

Build artifacts from `agents/drivers/<db-type>/build/libs/` are picked up by local driver install flows when available.

### Build

```bash
make package
```

The installer will be in `src-tauri/target/release/bundle/`.

## Tech Stack

| Layer     | Technology                                                                                                                                                                                                       |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Framework | [Tauri 2](https://tauri.app/)                                                                                                                                                                                    |
| Frontend  | [Vue 3](https://vuejs.org/) + TypeScript                                                                                                                                                                         |
| UI        | [shadcn-vue](https://www.shadcn-vue.com/) + Tailwind CSS                                                                                                                                                         |
| Editor    | [CodeMirror 6](https://codemirror.net/)                                                                                                                                                                          |
| Backend   | Rust + [sqlx](https://github.com/launchbadge/sqlx) / [tiberius](https://github.com/prisma/tiberius) / [redis-rs](https://github.com/redis-rs/redis-rs) / [mongodb](https://github.com/mongodb/mongo-rust-driver) |

## Documentation

- [Documentation source](https://github.com/ActiveInAI/PanDB/tree/main/docs) — feature guides and tutorials
- [Database Test Lab](https://github.com/ActiveInAI/PanDB/tree/main/deploy/database) — local database recipes for development and verification
- [Contributing](CONTRIBUTING.md) — how to pick up issues and open PRs
- [Web API reference](docs/content/docs/web-api.mdx) — HTTP API for Docker/Web deployments
- [Examples](examples/) — CLI, MCP, Docker, and API samples

## Community

<a href="https://discord.gg/W7NyVDRt6a" target="_blank"><img src="https://img.shields.io/badge/Discord-Join-5865F2?logo=discord&logoColor=white" alt="Discord" /></a>
<a href="https://qm.qq.com/q/1087880322" target="_blank"><img src="https://img.shields.io/badge/QQ%20群-1087880322-EB1923?logo=tencentqq&logoColor=white" alt="QQ Group" /></a>
<a href="https://docs.qq.com/doc/DVVhMY0h1ekJqc0tz" target="_blank"><img src="https://img.shields.io/badge/微信群-Join-07C160?logo=wechat&logoColor=white" alt="WeChat Group" /></a>
[![LINUX DO](https://img.shields.io/badge/LINUX%20DO-Community-blue)](https://linux.do)

## Support PanDB

PanDB is free and open source, but ongoing maintenance, database compatibility testing, infrastructure, and release work require sustained time and resources.

## FAQ

<details>
<summary><strong>Is PanDB free?</strong></summary>
Yes. PanDB is open source under Apache-2.0. All features are free.
</details>

<details>
<summary><strong>Does PanDB phone home?</strong></summary>
No. PanDB does not collect telemetry. The auto-update feature checks GitHub Releases for new versions — you can disable it in settings.
</details>

<details>
<summary><strong>Can I use PanDB without an internet connection?</strong></summary>
Yes. The desktop app works fully offline. For air-gapped driver installs, obtain the needed JDBC artifact on an internet-connected machine, transfer it to the offline machine, then import it in PanDB from Settings > Driver Manager. AI features need network access to the model endpoint (or a local model via Ollama).
</details>

<details>
<summary><strong>How is PanDB different from DBeaver / TablePlus / Beekeeper Studio?</strong></summary>
PanDB is 20 MB with no runtime dependencies for its native database features (no system Java or Python required). AI is built into the application, while MCP is provided as a separately installed Rust companion package or native binary. It supports 70+ databases across desktop, Docker, and web from a shared Rust core.
</details>

<details>
<summary><strong>What databases are supported?</strong></summary>
MySQL, PostgreSQL, SQLite, Cloudflare D1, Redis, MongoDB, DuckDB, ClickHouse, SQL Server, Oracle, Elasticsearch, Easysearch, Qdrant, Milvus, Weaviate, MariaDB, TiDB, OceanBase, openGauss, GaussDB, KWDB, KingBase, Vastbase, GoldenDB, Doris, SelectDB, StarRocks, Manticore Search, Redshift, DM, TDengine, XuguDB, CockroachDB, Access, HighGo, UXDB, and more. Agent/JDBC-oriented profiles extend support to H2, Snowflake, Trino, PrestoSQL, Hive, DB2, Informix, Neo4j, Cassandra, BigQuery, Kylin, SunDB, JDBCX, Databricks, SAP HANA, Teradata, Vertica, Firebird, Exasol, YashanDB, GBase 8a/8s, Databend, RQLite, Turso, InfluxDB, QuestDB, IoTDB, etcd, ZooKeeper, Nacos, IRIS, and custom JDBC connections. Message queue admin (Pulsar, Kafka, RocketMQ) is also supported.
</details>

<details>
<summary><strong>How do I report a bug or request a feature?</strong></summary>
Open an issue on <a href="https://github.com/ActiveInAI/PanDB/issues">GitHub Issues</a>.
</details>

## Contributors

<a href="https://github.com/ActiveInAI/PanDB/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ActiveInAI/PanDB" />
</a>

## Star History

<a href="https://www.star-history.com/?repos=ActiveInAI%2FPanDB&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=ActiveInAI/PanDB&type=date&legend=top-left" />
 </picture>
</a>

## License

[Apache-2.0](LICENSE)
