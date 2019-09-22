#! /usr/bin/env bash

set -e

DATA_DIR=$1
mkdir -p $1

# Generate a random text file of a given size in DATA_DIR
# Globals:
#   DATA_DIR
# Arguments:
#   file size in bytes
create_input_file() {
    base64 /dev/urandom | head -c $1 > $DATA_DIR/$1.txt
}

file_sizes=(
    1000
    10000
    100000
    500000
    1000000
    2000000
    5000000
    10000000
    20000000
    50000000
    100000000
    200000000
    500000000 # WARNING this takes a *long* time (over an hour)
)

read -r -p "Do you want to include the 0.5Gb benchmark? (takes > 60 mins) [y/N] " response
if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])+$ ]]
then
    unset 'file_sizes[${#file_sizes[@]}-1]'
fi

# set up test data files
echo "Creating test data files..."
for i in "${file_sizes[@]}"; do
    echo "$i bytes"
    create_input_file $i
done

# time each read_line implementation for each test file and output as .csv
now=$(date +%Y-%m-%d.%H:%M:%S)
outfile="$now-results.csv"
echo "input_file,read_line_version,time" >> $outfile
for f in $(ls -Sr $DATA_DIR); do
    echo "$f original..."
    utime="$( TIMEFORMAT='%U';time ( ./read_file --slow < $DATA_DIR/$f ) 2>&1 1>/dev/null )"
    echo $utime
    echo "$f,original,$utime" >> $outfile

    echo "$f improved..."
    utime="$( TIMEFORMAT='%U';time ( ./read_file < $DATA_DIR/$f ) 2>&1 1>/dev/null )"
    echo $utime
    echo "$f,improved,$utime" >> $outfile
done

echo "Results written to $outfile"