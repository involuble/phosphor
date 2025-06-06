#!/bin/sh

# --generate-inline-functions
# -- -DENABLE_STATIC_LIB
EMBREE_DIR="C:\Program Files\Intel\Embree3 x64"
export LIBCLANG_PATH="$(readlink -m ~/scoop/apps/llvm/current/bin)"
# CRT="C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\ucrt"
#CRT="C:\Program Files (x86)\Windows Kits\8.1\Include\shared"
#CRT="C:\Program Files (x86)\Windows Kits\8.1\Include\um"
# VC="C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.24.28314\include"
#VC="C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\include"
#VC="C:\Program Files\LLVM\lib\clang\5.0.0\include"
bindgen "$EMBREE_DIR\include\embree3\rtcore.h" -o pregenerated_bindings.rs \
    --no-layout-tests \
    --no-prepend-enum-name \
    --whitelist-function "rtc.*" \
    --whitelist-type "RTC.*" \
    --whitelist-var "RTC.*" \
    -- -I"$CRT" -I"$VC"