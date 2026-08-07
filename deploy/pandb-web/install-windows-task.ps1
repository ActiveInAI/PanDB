param(
  [string]$PackageRoot = (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)),
  [string]$TaskName = "PanDB Web"
)
$launcher = Join-Path $PackageRoot "run-pandb-web.cmd"
if (-not (Test-Path -LiteralPath $launcher)) { throw "Missing launcher: $launcher" }
$action = New-ScheduledTaskAction -Execute "cmd.exe" -Argument ("/c `"$launcher`"") -WorkingDirectory $PackageRoot
$trigger = New-ScheduledTaskTrigger -AtStartup
$principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType InteractiveToken -RunLevel Limited
Register-ScheduledTask -TaskName $TaskName -Action $action -Trigger $trigger -Principal $principal -Force | Out-Null
Write-Host "Installed $TaskName for $PackageRoot"
