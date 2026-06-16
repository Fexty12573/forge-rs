#!/bin/bash

cargo publish -p mhgu-forge-macros
if [ $? -ne 0 ]; then
    echo "Failed to publish mhgu-forge-macros"
    exit 1
fi

cargo publish -p mhgu-forge-sys
if [ $? -ne 0 ]; then
    echo "Failed to publish mhgu-forge-sys"
    exit 1
fi

cargo publish -p mhgu-forge-rt
if [ $? -ne 0 ]; then
    echo "Failed to publish mhgu-forge-rt"
    exit 1
fi

cargo publish -p mhgu-forge
if [ $? -ne 0 ]; then
    echo "Failed to publish mhgu-forge"
    exit 1
fi

cargo publish -p cargo-mhgu-forge
if [ $? -ne 0 ]; then
    echo "Failed to publish cargo-mhgu-forge"
    exit 1
fi
