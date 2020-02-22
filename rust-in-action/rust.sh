#!/bin/bash
# Compile and run a one-off Rust program.
# Used for running standalone scripts/examples from the book
name=$(basename $1 .rs)
rustc $@ && ./$name && rm $name
