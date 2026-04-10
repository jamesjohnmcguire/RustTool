@ECHO OFF
@ECHO You will need to have msbuild.exe on your path.

CD %~dp0
CD ..

cargo clean
cargo build

:end
IF /I "%1"=="clean" GOTO clean
GOTO :EOF

:clean
cargo clean
