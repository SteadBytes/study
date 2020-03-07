#! /usr/bin/env bash

set -e

EXE=target/debug/particles
TRACE_TIME=20
TRACE_FILE=trace.txt
ALLOCS_TSV=allocs.tsv
ALLOCS_GRAPH=allocs.png

# Ensure executable is available for tracing
cargo build

# Trace execution
timeout $TRACE_TIME ltrace -T -o $TRACE_FILE -e "-*+malloc+free+realloc" ./$EXE

# Extract malloc calls into a .tsv file
sed 's/->/ /' $TRACE_FILE | \
	sed 's/, /|/' | \
	tr '()\><=' ' ' | \
	column -t | \
	tr -s ' ' '\t' | \
	grep malloc > $ALLOCS_TSV

# Plot
gnuplot $ALLOCS_TSV > $ALLOCS_GRAPH
