#!/bin/bash

# 引数をチェック
if [ "$#" -lt 1 ]; then
    echo "少なくとも1つの引数が必要です"
    exit 1
fi

# gccでコンパイル
gcc $(for arg in "$@"; do echo "sample-codes/${arg}.c"; done) -o sample-codes/exe/$1
