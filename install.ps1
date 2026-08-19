# envo installer for Windows PowerShell.
#
#   irm https://raw.githubusercontent.com/kaihere14/climenv/main/install.ps1 | iex
#
# Downloads the matching binary from the latest GitHub release, verifies its
# checksum and drops it on your PATH. Works in Windows PowerShell 5.1 and
# PowerShell 7 with no curl, tar, unzip or Git Bash needed, so it runs in the
# terminal every Windows IDE opens by default.
#
# Environment:
#   ENVO_VERSION      release tag to install (default: latest)
#   ENVO_INSTALL_DIR  where to put the binary (default: %LOCALAPPDATA%\envo\bin)
#
# Piped into `iex` there is no way to pass parameters, so both settings are
# read from the environment:
#   $env:ENVO_VERSION = 'v0.1.0'; irm <url> | iex

$ErrorActionPreference = "Stop"

$Repo = "kaihere14/climenv"
$Bin = "envo"
$Version = if ($env:ENVO_VERSION) { $env:ENVO_VERSION } else { "latest" }
$InstallDir = if ($env:ENVO_INSTALL_DIR) { $env:ENVO_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "envo\bin" }

# Same symbols the CLI itself uses, so the install reads like an envo run.
function Write-Step { param([string]$Message) Write-Host "- $Message" }
function Write-Success { param([string]$Message) Write-Host "$([char]0x2713) $Message" }
function Write-Warn { param([string]$Message) [Console]::Error.WriteLine("! $Message") }
function Stop-Fail {
    param([string]$Message)
    [Console]::Error.WriteLine("X $Message")
    exit 1
}

# Windows PowerShell 5.1 still defaults to TLS 1.0/1.1, which GitHub refuses.
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# Only one Windows target is published. ARM64 machines run the x64 build
# under emulation, so install it rather than failing.
$arch = $env:PROCESSOR_ARCHITECTURE
if ($env:PROCESSOR_ARCHITEW6432) { $arch = $env:PROCESSOR_ARCHITEW6432 }

switch ($arch) {
    "AMD64" { }
    "ARM64" { Write-Warn "no native ARM64 build yet - installing the x86_64 binary, which runs under emulation" }
    default { Stop-Fail "unsupported CPU architecture: $arch" }
}

$Target = "x86_64-pc-windows-msvc"
$Asset = "$Bin-$Target.zip"
$BinFile = "$Bin.exe"

if ($Version -eq "latest") {
    # This path redirects to the newest release, so no API call and no rate
    # limit for unauthenticated installs.
    $BaseUrl = "https://github.com/$Repo/releases/latest/download"
} else {
    $BaseUrl = "https://github.com/$Repo/releases/download/$Version"
}

$Tmp = Join-Path ([IO.Path]::GetTempPath()) ("envo-install-" + [Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $Tmp | Out-Null

try {
    $ArchivePath = Join-Path $Tmp $Asset
    $ChecksumPath = "$ArchivePath.sha256"

    Write-Step "Downloading $Bin $Version for $Target"

    # A progress bar makes Invoke-WebRequest an order of magnitude slower on
    # Windows PowerShell, and it renders as garbage in non-interactive shells.
    $progress = $ProgressPreference
    $ProgressPreference = "SilentlyContinue"
    try {
        try {
            Invoke-WebRequest -Uri "$BaseUrl/$Asset" -OutFile $ArchivePath -UseBasicParsing
        } catch {
            Stop-Fail "could not download $BaseUrl/$Asset - check that a release exists for $Version"
        }
        Invoke-WebRequest -Uri "$BaseUrl/$Asset.sha256" -OutFile $ChecksumPath -UseBasicParsing
    } finally {
        $ProgressPreference = $progress
    }

    # sha256sum format: "<hash>  <file>".
    $expected = ((Get-Content -Raw $ChecksumPath).Trim() -split '\s+')[0].ToLower()
    $actual = (Get-FileHash -Algorithm SHA256 $ArchivePath).Hash.ToLower()
    if ($expected -ne $actual) {
        Stop-Fail "checksum mismatch: expected $expected, got $actual"
    }

    Expand-Archive -Path $ArchivePath -DestinationPath $Tmp -Force

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    # ENVO_INSTALL_DIR may be relative; the PATH entry must not be.
    $InstallDir = (Resolve-Path -LiteralPath $InstallDir).Path
    $Dest = Join-Path $InstallDir $BinFile

    # A running envo would hold a lock on the file it was launched from.
    try {
        Move-Item -Path (Join-Path $Tmp $BinFile) -Destination $Dest -Force
    } catch {
        Stop-Fail "could not write $Dest - close any running $Bin and try again: $($_.Exception.Message)"
    }

    Write-Success "Installed $Bin to $Dest"
} finally {
    Remove-Item -Recurse -Force $Tmp -ErrorAction SilentlyContinue
}

# The user PATH in the registry is what new terminals inherit; $env:Path only
# covers this session. Both are updated, so `envo` works in this window and in
# every one opened afterwards.
#
# The registry value is read and written raw: [Environment]::GetEnvironmentVariable
# expands %VARS% and writing that back would flatten a REG_EXPAND_SZ PATH into
# literal text, so entries the user wrote as %USERPROFILE%\... would stop
# tracking the variable.
function Add-ToUserPath {
    param([string]$Directory)

    $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey("Environment", $true)
    if (-not $key) { throw "could not open HKCU\Environment" }

    try {
        $current = $key.GetValue("Path", "", [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
        $entries = @($current -split ';' | Where-Object { $_ -ne "" })

        # An entry already pointing here may be written as %LOCALAPPDATA%\envo\bin,
        # so compare expanded too rather than adding a second copy on re-install.
        $target = $Directory.TrimEnd('\')
        $existing = @($entries | ForEach-Object { [Environment]::ExpandEnvironmentVariables($_).TrimEnd('\') })
        if ($existing -contains $target) { return $false }

        try { $kind = $key.GetValueKind("Path") } catch { $kind = [Microsoft.Win32.RegistryValueKind]::ExpandString }
        $key.SetValue("Path", (($entries + $Directory) -join ';'), $kind)
    } finally {
        $key.Close()
    }

    # A registry write alone is invisible to already-running processes.
    # Explorer picks the new PATH up from this broadcast, so terminals it
    # launches later inherit it without a sign-out. The PATH is already
    # written at this point, so a failing broadcast is not a failing install.
    try {
        if (-not ("Envo.Native" -as [type])) {
            Add-Type -Namespace "Envo" -Name "Native" -MemberDefinition @"
[System.Runtime.InteropServices.DllImport("user32.dll", SetLastError = true, CharSet = System.Runtime.InteropServices.CharSet.Auto)]
public static extern System.IntPtr SendMessageTimeout(System.IntPtr hWnd, uint Msg, System.UIntPtr wParam, string lParam, uint fuFlags, uint uTimeout, out System.UIntPtr lpdwResult);
"@
        }

        $HWND_BROADCAST = [IntPtr]0xffff
        $WM_SETTINGCHANGE = 0x1a
        $SMTO_ABORTIFHUNG = 0x2
        $result = [UIntPtr]::Zero
        [void][Envo.Native]::SendMessageTimeout($HWND_BROADCAST, $WM_SETTINGCHANGE, [UIntPtr]::Zero, "Environment", $SMTO_ABORTIFHUNG, 5000, [ref]$result)
    } catch {
        Write-Warn "PATH updated, but other running programs will not see it until they restart"
    }

    return $true
}

try {
    if (Add-ToUserPath $InstallDir) {
        Write-Success "Added $InstallDir to your PATH"
    }
} catch {
    Write-Warn "could not add $InstallDir to your PATH - add it under System Properties > Environment Variables: $($_.Exception.Message)"
}

if (($env:Path -split ';') -notcontains $InstallDir) {
    $env:Path = "$env:Path;$InstallDir"
    Write-Warn "terminals and IDEs that were already open need a restart to find $Bin"
}

Write-Step "Run ``$Bin keygen`` to create your identity"
