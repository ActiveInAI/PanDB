# PanDB Web 持久化运行

这些模板随 Web 后端包分发，不会自动修改当前主机服务。

## Ubuntu x64 / arm64

将 `bin/dbx-web-bin` 和 `dist/` 放入 `/opt/pandb`，创建用户 `pandb`，复制 `pandb.env.example` 到 `/etc/pandb/pandb.env` 并修改密码，然后安装 `pandb-web.service` 到 `/etc/systemd/system/`，执行 `systemctl enable --now pandb-web`。

## Windows x64

将后端可执行文件命名为 `bin\\dbx-web.exe`，先在 `run-pandb-web.cmd` 中设置密码，再以管理员 PowerShell 执行 `install-windows-task.ps1`。它创建开机启动的当前用户计划任务。
