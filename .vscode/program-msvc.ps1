param(
  [Parameter(Mandatory=$true)]
  [string] $BinName
)

$ErrorActionPreference = "Stop"
. "$PSScriptRoot\msvc-env.ps1"

# Run the built binary from target\debug
$exe = Join-Path $PSScriptRoot "..\target\debug\$BinName.exe"
$exe = (Resolve-Path $exe).Path

# Run from workspace root so relative paths are stable
$workspace = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $workspace
try {
  & $exe
  exit $LASTEXITCODE
}
finally {
  Pop-Location
}
