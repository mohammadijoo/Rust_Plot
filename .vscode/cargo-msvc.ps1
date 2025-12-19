param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$here = Split-Path -Parent $MyInvocation.MyCommand.Path
. (Join-Path $here "msvc-env.ps1")

if (-not $CargoArgs -or $CargoArgs.Count -eq 0) {
  $CargoArgs = @("build")
}

Write-Host "Running: cargo $($CargoArgs -join ' ')"
& cargo @CargoArgs
exit $LASTEXITCODE
