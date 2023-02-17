#!/usr/bin/bash

#!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#!!!    This script was made for me, MCorange     !!!
#!!!  And it will probably not work for you,      !!!
#!!!  i use arch so good luck if wanna try        !!!
#!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

# linux
cargo build --release

# win32
cargo build --release --target "x86_64-pc-windows-gnu" \
                            --config x86_64-pc-windows-gnu.linker=\"/usr/bin/x86_64-w64-mingw32-gcc\" \
                            --config x86_64-pc-windows-gnu.ar=\"/usr/bin/x86_64-w64-mingw32-ar\"

cp ./target/release/svg2colored-png ./target/svg2colored-png-linux-x86_64
cp ./target/x86_64-pc-windows-gnu/release/svg2colored-png.exe ./target/svg2colored-png-win32-x86_64.exe
                            