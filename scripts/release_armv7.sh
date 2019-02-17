#!/bin/bash

cargo build --target armv7-unknown-linux-gnueabihf --release

if [ -d "releases/armv7" ]; then
    rm -r releases/armv7
fi
mkdir -p releases/armv7
cp target/armv7-unknown-linux-gnueabihf/release/rust_iron_practice releases/armv7/
cp -r public releases/armv7/
cp -r templates releases/armv7/
zip -r releases/armv7.zip releases/armv7
