$ErrorActionPreference = "Stop"

$vsRoot   = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\2022\BuildTools"
$msvcRoot = Join-Path $vsRoot "VC\Tools\MSVC"

if (!(Test-Path $msvcRoot)) {
  throw "MSVC toolsets folder not found: $msvcRoot"
}

# Pick newest toolset that has cl/link + MSVC headers (stddef.h)
$toolsets = Get-ChildItem $msvcRoot -Directory | Sort-Object Name -Descending
$selected = $null

foreach ($t in $toolsets) {
  $cl     = Join-Path $t.FullName "bin\Hostx64\x64\cl.exe"
  $link   = Join-Path $t.FullName "bin\Hostx64\x64\link.exe"
  $stddef = Join-Path $t.FullName "include\stddef.h"
  if ((Test-Path $cl) -and (Test-Path $link) -and (Test-Path $stddef)) {
    $selected = $t
    break
  }
}

if (-not $selected) {
  throw "No MSVC toolset found with headers. Install/repair: 'MSVC v143 - VS 2022 C++ x64/x86 build tools' + Windows 10/11 SDK."
}

$msvcVer = $selected.Name
$msvcDir = $selected.FullName
$msvcBin = Join-Path $msvcDir "bin\Hostx64\x64"
$msvcInc = Join-Path $msvcDir "include"
$msvcLib = Join-Path $msvcDir "lib\x64"

# Windows SDK (pick newest)
$sdkLibRoot     = Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10\Lib"
$sdkIncludeRoot = Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10\Include"

$sdkDirs = Get-ChildItem $sdkLibRoot -Directory | Sort-Object Name -Descending
if ($sdkDirs.Count -eq 0) { throw "No Windows SDK found under: $sdkLibRoot" }

$sdkVer = $sdkDirs[0].Name

$ucrtInc     = Join-Path $sdkIncludeRoot "$sdkVer\ucrt"
$umInc       = Join-Path $sdkIncludeRoot "$sdkVer\um"
$sharedInc   = Join-Path $sdkIncludeRoot "$sdkVer\shared"
$winrtInc    = Join-Path $sdkIncludeRoot "$sdkVer\winrt"
$cppwinrtInc = Join-Path $sdkIncludeRoot "$sdkVer\cppwinrt"

$ucrtLib = Join-Path $sdkLibRoot "$sdkVer\ucrt\x64"
$umLib   = Join-Path $sdkLibRoot "$sdkVer\um\x64"

# Sanity checks
if (!(Test-Path (Join-Path $msvcInc "stddef.h")))     { throw "MSVC headers missing: $msvcInc (stddef.h not found)" }
if (!(Test-Path (Join-Path $umLib "kernel32.lib")))   { throw "Windows SDK libs missing: $umLib (kernel32.lib not found)" }
if (!(Test-Path (Join-Path $msvcLib "msvcrt.lib")))   { throw "MSVC libs missing: $msvcLib (msvcrt.lib not found)" }

# Force Cargo linker (session-only)
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = (Join-Path $msvcBin "link.exe")

# INCLUDE/LIB for cl.exe and link.exe
$env:INCLUDE = "$msvcInc;$ucrtInc;$umInc;$sharedInc;$winrtInc;$cppwinrtInc;$env:INCLUDE"
$env:LIB     = "$msvcLib;$ucrtLib;$umLib;$env:LIB"

# Helpful vars
$env:VCToolsInstallDir   = "$msvcDir\"
$env:VCINSTALLDIR        = (Join-Path $vsRoot "VC") + "\"
$env:WindowsSdkDir       = (Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10") + "\"
$env:WindowsSDKVersion   = "$sdkVer\"
$env:UniversalCRTSdkDir  = (Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10") + "\"

# PATH hygiene: put MSVC first; remove MSYS/Git usr\bin shadowing
$pathParts = $env:Path -split ";" | Where-Object { $_ -and $_.Trim() -ne "" }
$clean = @()
foreach ($p in $pathParts) {
  if ($p -match "\\msys64\\usr\\bin\\?$") { continue }
  if ($p -match "\\Program Files\\Git\\usr\\bin\\?$") { continue }
  $clean += $p
}
$env:Path = "$msvcBin;" + ($clean -join ";")

Write-Host "MSVC env loaded."
Write-Host "  Toolset: $msvcVer"
Write-Host "  SDK:     $sdkVer"
Write-Host "  Linker:  $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER"
