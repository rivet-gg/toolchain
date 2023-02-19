#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# Create bin directory for Rivet
$RivetInstall = $env:RIVET_INSTALL
$BinDir = if ($RivetInstall) {
	"${RivetInstall}\bin"
} else {
	"${Home}\.rivet\bin"
}

if (!(Test-Path $BinDir)) {
	New-Item $BinDir -ItemType Directory | Out-Null
}

$RivetZip = "$BinDir\rivet.zip"
$RivetExe = "$BinDir\rivet.exe"
$Target = 'x86_64-pc-windows-msvc'
$CliAssetSuffix = "-${Target}.zip"

# Auto-select version to install
#
# We have to find last version with an asset so we don't break the installer
# when the assets for a new version are still generating
$Version = $env:RIVET_CLI_VERSION
if (!$Version) {
	$Releases = Invoke-RestMethod -Uri "https://api.github.com/repos/rivet-gg/cli/releases"

	foreach ($Release in $Releases) {
		$SelectedAssets = $Release.assets | Select-Object -ExpandProperty name | Where-Object { $_ -like "*$CliAssetSuffix" }
		if ($SelectedAssets) {
			$Version = $Release.name
			Break
		}
	}

	if (!$Version) {
		throw 'Failed to determine version to install'
	}
}

Write-Host
Write-Host "> Installing Rivet CLI @ ${Version}"

# Download CLI
$DownloadUrl = "https://github.com/rivet-gg/cli/releases/download/${Version}/rivet-${Version}-${Target}.zip"
Write-Host
Write-Host "> Downloading ${DownloadUrl}"
Invoke-WebRequest $DownloadUrl -OutFile $RivetZip -UseBasicParsing

# Extract archive
Write-Host
Write-Host "> Extracting rivet.zip"
if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
	Expand-Archive $RivetZip -Destination $BinDir -Force
} else {
	Remove-Item $RivetExe -ErrorAction SilentlyContinue
	Add-Type -AssemblyName System.IO.Compression.FileSystem
	[IO.Compression.ZipFile]::ExtractToDirectory($RivetZip, $BinDir)
}
Remove-Item $RivetZip

# Install CLI
Write-Host
Write-Host "> Installing rivet"
$User = [System.EnvironmentVariableTarget]::User
$Path = [System.Environment]::GetEnvironmentVariable('Path', $User)
if (!(";${Path};".ToLower() -like "*;${BinDir};*".ToLower())) {
	[System.Environment]::SetEnvironmentVariable('Path', "${Path};${BinDir}", $User)
	$Env:Path += ";${BinDir}"
}

Write-Host
Write-Host "> Checking installation"
rivet.exe --version

Write-Host
Write-Host "Rivet was installed successfully to ${RivetExe}."
Write-Host "Run 'rivet --help' to get started."
Write-Host
Write-Host "If the 'rivet' command is not found, open a new shell."
Write-Host
