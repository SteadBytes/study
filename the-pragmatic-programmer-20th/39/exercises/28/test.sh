#! /usr/bin/env bash

set -e

OUTPUT_DIR=$1

ROOT_DIR=`pwd`
PRG_DIR=sort
BIN=$PRG_DIR/target/release/sort
MANIFEST_FILE=$PRG_DIR/Cargo.toml
OPTIMIZATION_CONFIG_KEY=opt-level

# replace value in 'key = value' format
function replace_config_value {
    declare target_key="$1" replacement_value="$2"
    sed -i "s/\($target_key *= *\).*/\1$replacement_value/" $MANIFEST_FILE
}

for i in {0..3} # optimization levels supported by rustc
do
    echo "Building with optimization level $i"
    replace_config_value $OPTIMIZATION_CONFIG_KEY "$i"
    cargo build --release --manifest-path=$MANIFEST_FILE
    ./$BIN | tee $OUTPUT_DIR/sort-$i.out
done