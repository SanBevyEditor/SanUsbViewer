$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath $PSScriptRoot
function Invoke-ProjectCargo {
    if (Get-Command rtk -ErrorAction SilentlyContinue) { rtk proxy cargo @args }
    else { & cargo @args }
}
$target = 'x86_64-pc-windows-msvc'
$previousFlags = $env:CARGO_ENCODED_RUSTFLAGS
try {
    # Embed the C runtime and replace private build paths in panic messages.
    $flags = @('-C', 'target-feature=+crt-static', "--remap-path-prefix=$PSScriptRoot=/app")
    if ($env:USERPROFILE) { $flags += "--remap-path-prefix=$env:USERPROFILE=/build-user" }
    if ($env:CARGO_HOME) { $flags += "--remap-path-prefix=$env:CARGO_HOME=/cargo" }
    $env:CARGO_ENCODED_RUSTFLAGS = $flags -join [char]31
    Invoke-ProjectCargo build --release --locked --target $target
    if ($LASTEXITCODE -ne 0) { throw 'La compilation a echoue.' }
}
finally { $env:CARGO_ENCODED_RUSTFLAGS = $previousFlags }
$destination = Join-Path $PSScriptRoot ('dist/staging/' + [guid]::NewGuid().ToString('N') + '/SanUsbViewer')
New-Item -ItemType Directory -Force -Path $destination | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot "target/$target/release/san-usb-viewer.exe") -Destination (Join-Path $destination 'SanUsbViewer.exe')
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'README.md') -Destination $destination
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'LICENSE') -Destination $destination
$metadataJson = Invoke-ProjectCargo metadata --format-version 1 --locked
if ($LASTEXITCODE -ne 0) { throw 'Lecture des licences des dependances impossible.' }
$metadata = ($metadataJson -join "`n") | ConvertFrom-Json
$project = $metadata.packages | Where-Object { $_.name -eq 'san-usb-viewer' }
$version = $project.version
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
$packageFiles = @('SanUsbViewer.exe', 'README.md', 'LICENSE', 'THIRD-PARTY-NOTICES.txt', 'FONT-LICENSE.txt', 'FONT-CJK-LICENSE.txt', 'FONT-SOURCES.json', 'VERSION.txt', 'START-HERE.txt')
[System.IO.File]::WriteAllText((Join-Path $destination 'VERSION.txt'), "San USB Viewer $version`nWindows 10/11 x64`nTarget: $target`n", $utf8)
$instructions = @"
San USB Viewer $version - Windows 10/11 x64

FR : Extrayez toute l'archive, puis ouvrez SanUsbViewer.exe.
Aucune installation de Rust ou de pilote USB n'est necessaire.
Cliquez sur le code de langue en haut a droite pour choisir parmi 50 langues.
README.md et docs/README.md contiennent les instructions et les limites connues.

EN: Extract the entire archive, then open SanUsbViewer.exe.
No Rust installation or USB driver installation is required.
Click the language code at the top right to choose from 50 languages.
See README.md and docs/README.md for documentation and known limitations.

https://github.com/SanBevyEditor/SanUsbViewer
"@
[System.IO.File]::WriteAllText((Join-Path $destination 'START-HERE.txt'), $instructions.Replace("`r`n", "`n") + "`n", $utf8)
$documentation = @('docs/README.md', 'assets/screenshots/SanUsbViewer.jpg')
$languages = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'locales/languages.json') -Raw -Encoding UTF8 | ConvertFrom-Json
foreach ($language in $languages) { $documentation += "docs/readme/README.$($language.code).md" }
foreach ($relativePath in $documentation) {
    $outputPath = Join-Path $destination $relativePath
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $outputPath) | Out-Null
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot $relativePath) -Destination $outputPath
    $packageFiles += $relativePath
}
# An explicit file list keeps local inventories or stale files out of releases.
Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem
$archiveName = "SanUsbViewer-v$version-windows-x86_64.zip"
$archivePath = Join-Path $PSScriptRoot "dist/$archiveName"
$stream = [System.IO.File]::Open($archivePath, [System.IO.FileMode]::Create)
try {
    $archive = New-Object System.IO.Compression.ZipArchive($stream, [System.IO.Compression.ZipArchiveMode]::Create, $true)
    try {
        foreach ($relativePath in ($packageFiles | Sort-Object -Unique)) {
            $entry = $archive.CreateEntry("SanUsbViewer/$relativePath", [System.IO.Compression.CompressionLevel]::Optimal)
            $entry.LastWriteTime = [DateTimeOffset]::new(2000, 1, 1, 0, 0, 0, [TimeSpan]::Zero)
            $inputStream = [System.IO.File]::OpenRead((Join-Path $destination $relativePath))
            try {
                $entryStream = $entry.Open()
                try { $inputStream.CopyTo($entryStream) }
                finally { $entryStream.Dispose() }
            }
            finally { $inputStream.Dispose() }
        }
    }
    finally { $archive.Dispose() }
}
finally { $stream.Dispose() }
$hasher = [System.Security.Cryptography.SHA256]::Create()
$inputStream = [System.IO.File]::OpenRead($archivePath)
try { $checksum = [BitConverter]::ToString($hasher.ComputeHash($inputStream)).Replace('-', '').ToLowerInvariant() }
finally { $inputStream.Dispose(); $hasher.Dispose() }
[System.IO.File]::WriteAllText((Join-Path $PSScriptRoot 'dist/SHA256SUMS.txt'), "$checksum  $archiveName`n", $utf8)
Write-Output "Application prete : $destination/SanUsbViewer.exe"
Write-Output "Distribution : $archivePath"
Write-Output "SHA-256 : $checksum"
