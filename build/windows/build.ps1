# Builds the Windows release binary and packages it the way install.sh
# expects: dist\envo-<target>.zip plus a matching .sha256.
#
#   pwsh -File build/windows/build.ps1 [-Target <triple>]

param(
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

$Bin  = "envo"
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$Dist = Join-Path $Root "dist"

Set-Location $Root

if (Get-Command rustup -ErrorAction SilentlyContinue) {
    rustup target add $Target | Out-Null
}

Write-Host "- Building $Bin for $Target"
cargo build --release --locked --target $Target
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

New-Item -ItemType Directory -Force -Path $Dist | Out-Null

$Archive    = "$Bin-$Target.zip"
$ArchivePath = Join-Path $Dist $Archive
$ExePath     = Join-Path $Root "target\$Target\release\$Bin.exe"

Compress-Archive -Path $ExePath -DestinationPath $ArchivePath -Force

# Written in the same "<hash>  <file>" shape as sha256sum/shasum so the one
# checksum check in install.sh works for every platform.
$Hash = (Get-FileHash -Algorithm SHA256 $ArchivePath).Hash.ToLower()
"$Hash  $Archive" | Out-File -FilePath "$ArchivePath.sha256" -Encoding ascii -NoNewline

Write-Host "✓ Packaged dist/$Archive"
