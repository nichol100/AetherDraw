; Inno Setup script for AetherDraw (Windows)
[Setup]
AppName=AetherDraw
AppVersion=0.1.0
DefaultDirName={autopf}\AetherDraw
DefaultGroupName=AetherDraw
OutputBaseFilename=aetherdraw-setup
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "target\release\aetherdraw.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\AetherDraw"; Filename: "{app}\aetherdraw.exe"
Name: "{autodesktop}\AetherDraw"; Filename: "{app}\aetherdraw.exe"

[Run]
Filename: "{app}\aetherdraw.exe"; Description: "Launch AetherDraw"; Flags: nowait postinstall skipifsilent