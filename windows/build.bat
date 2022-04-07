@echo off

echo Building...
rustup run stable-i686-pc-windows-gnu cargo build --release
echo Building done

set EXT_PATH=C:\Users\Dan\Documents\GameMaker\Projects\ld50.gmx\extensions
set EXT_PATH_OTHER=C:\Users\Dan\Documents\GameMaker\Projects\ld50.gmx\extensions
set DROP_PATH=C:\Users\Dan\ld50_lib\windows\target\release

echo Copying to %EXT_PATH%
del "%EXT_PATH%\ld50_lib.extension.gmx"
del "%EXT_PATH%\ld50_lib\ld50_lib.dll"
copy "%DROP_PATH%\ld50_lib_windows.dll" "%EXT_PATH%\ld50_lib\ld50_lib.dll"
REM move "%EXT_PATH%\ld50_lib\windows_lib.dll" "%EXT_PATH%\wld50_lib\orld_generators.dll"
copy "C:\users\Dan\tmp\ld50_lib.xml" "%EXT_PATH%"
move "%EXT_PATH%\ld50_lib.xml" "%EXT_PATH%\ld50_lib.extension.gmx"
echo Done