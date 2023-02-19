#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$RivetZip = "$BinDir\rivet.zip"
$RivetExe = "$BinDir\rivet.exe"
$Target = 'x86_64-pc-windows-msvc'

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

# Determine version
$Version = $env:RIVET_CLI_VERSION
if (!$Version) {
	$Version = "v0.0.40"
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

if (!(Test-Path $RivetExe)) {
  Start-Process -FilePath "$env:comspec" -ArgumentList "/c", "mklink", $RivetExe -Verb runAs
}

Write-Host
Write-Host "> Checking installation"
rivet.exe --version

Write-Host
Write-Output "Rivet was installed successfully to ${RivetExe}."
Write-Output "Run 'rivet --help' to get started."
