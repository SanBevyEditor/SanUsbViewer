$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath $PSScriptRoot
function Invoke-ProjectCargo {
    if (Get-Command rtk -ErrorAction SilentlyContinue) { rtk proxy cargo @args }
    else { & cargo @args }
}
Invoke-ProjectCargo build --release --locked
if ($LASTEXITCODE -ne 0) { throw 'La compilation a echoue.' }
$destination = Join-Path $PSScriptRoot 'dist/SanUsbViewer'
New-Item -ItemType Directory -Force -Path $destination | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target/release/san-usb-viewer.exe') -Destination (Join-Path $destination 'SanUsbViewer.exe')
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'README.md') -Destination $destination
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'LICENSE') -Destination $destination
$metadataJson = Invoke-ProjectCargo metadata --format-version 1 --locked
if ($LASTEXITCODE -ne 0) { throw 'Lecture des licences des dependances impossible.' }
$metadata = ($metadataJson -join "`n") | ConvertFrom-Json
$notices = New-Object System.Text.StringBuilder
[void]$notices.AppendLine('San USB Viewer - Third-party dependency notices')
[void]$notices.AppendLine('Includes the complete resolved dependency graph, including optional/platform-specific packages.')
foreach ($package in ($metadata.packages | Sort-Object name,version)) {
    if ($package.name -eq 'san-usb-viewer') { continue }
    [void]$notices.AppendLine("`n============================================================")
    [void]$notices.AppendLine("$($package.name) $($package.version) - $($package.license)")
    [void]$notices.AppendLine([string]$package.repository)
    $packageRoot = Split-Path -Parent $package.manifest_path
    $licenseFiles = @(Get-ChildItem -LiteralPath $packageRoot -File | Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE)([-_.].*)?$' })
    if ($package.license_file) {
        $declared = Join-Path $packageRoot $package.license_file
        if (Test-Path -LiteralPath $declared) { $licenseFiles += Get-Item -LiteralPath $declared }
    }
    foreach ($licenseFile in ($licenseFiles | Sort-Object FullName -Unique)) {
        [void]$notices.AppendLine("`n--- $($licenseFile.Name) ---")
        [void]$notices.AppendLine((Get-Content -LiteralPath $licenseFile.FullName -Raw -Encoding UTF8))
    }
}
$utf8 = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText((Join-Path $destination 'THIRD-PARTY-NOTICES.txt'), $notices.ToString(), $utf8)
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'assets/fonts/OFL.txt') -Destination (Join-Path $destination 'FONT-LICENSE.txt')
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'assets/fonts/OFL-CJK.txt') -Destination (Join-Path $destination 'FONT-CJK-LICENSE.txt')
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'assets/fonts/sources.json') -Destination (Join-Path $destination 'FONT-SOURCES.json')
Write-Output "Application prete : $destination/SanUsbViewer.exe"
