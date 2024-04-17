#!/usr/bin/env bash

RELEASE_MODE=${1}
LIB_FOLDER="debug"

# build to Android target
if [ "${RELEASE_MODE}" = "--release" ]; then
    LIB_FOLDER="release"
    echo "BUILDING FOR aarch64-linux-android release"
    cargo so b --lib --target aarch64-linux-android  ${RELEASE_MODE}
    
    echo "BUILDING FOR x86_64-linux-android release"
    cargo so b --lib --target x86_64-linux-android  ${RELEASE_MODE}
else
    echo "BUILDING FOR aarch64-linux-android debug"
    RUST_BACKTRACE=full RUST_LOG=wgpu_hal=debug cargo so b --lib --target aarch64-linux-android

    echo "BUILDING FOR x86_64-linux-android debug"
    RUST_BACKTRACE=full RUST_LOG=wgpu_hal=debug cargo so b --lib --target x86_64-linux-android
fi

# copy .so files to jniLibs folder
ARM64="Android/app/libs/arm64-v8a"
ARMv7a="Android/app/libs/armeabi-v7a"
x86_64="Android/app/libs/x86_64"

if [ ! -d "$ARM64" ]; then
    mkdir -p "$ARM64"
fi
if [ ! -d "$ARMv7a" ]; then
    mkdir -p "$ARMv7a"
fi
if [ ! -d "$x86_64" ]; then
    mkdir -p "$x86_64"
fi

cp target/aarch64-linux-android/${LIB_FOLDER}/libbevy_in_app.so "${ARM64}/libbevy_in_app.so"

cp target/x86_64-linux-android/${LIB_FOLDER}/libbevy_in_app.so "${x86_64}/libbevy_in_app.so"
