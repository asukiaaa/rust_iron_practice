#!/bin/bash
PROJECT_NAME=rust_iron_practice
BUILD_TARGET=arm-unknown-linux-gnueabi
RELEASE_DIR=armv6

cargo build --target ${BUILD_TARGET} --release

if [ -d "releases/${RELEASE_DIR}" ]; then
    rm -r releases/${RELEASE_DIR}
fi
mkdir -p releases/${RELEASE_DIR}
cp target/${BUILD_TARGET}/release/${PROJECT_NAME} releases/${RELEASE_DIR}/
cp -r public releases/${RELEASE_DIR}/
cp -r templates releases/${RELEASE_DIR}/
cd releases
if [ -f "${RELEASE_DIR}.zip" ]; then
    rm ${RELEASE_DIR}.zip
fi
zip -r ${RELEASE_DIR}.zip ${RELEASE_DIR}
cd ../
