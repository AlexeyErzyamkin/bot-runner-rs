@echo off

echo Building

cargo build --release

echo Copying MASTER files

mkdir bin\master\data

copy /y target\release\master.exe bin\master
rem copy /y data\master_config.json bin\master\data

echo Copying WORKER files

mkdir bin\worker\data

copy /y target\release\worker.exe bin\worker
rem copy /y data\worker_config.json bin\worker\data