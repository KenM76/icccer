# Builds lcms2_black_point.exe against the pinned lcms2 that build-lcms2.ps1
# produced. Output: vendor/build-msvc/ (gitignored). README §29.
$ErrorActionPreference = 'Stop'
$vendor = Join-Path $PSScriptRoot '..\vendor'
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
$vcvars = Join-Path $vs 'VC\Auxiliary\Build\vcvars64.bat'
$out = Join-Path $vendor 'build-msvc'
$cl = "cl /nologo /O2 /MD /I`"$vendor\lcms2\include`" `"$PSScriptRoot\lcms2_black_point.c`" `"$out\lcms2.lib`" /Fo`"$out\lcms2_black_point.obj`" /Fe`"$out\lcms2_black_point.exe`""
cmd /c "`"$vcvars`" >nul && $cl"
if ($LASTEXITCODE -ne 0) { exit 4 }
