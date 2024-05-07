#!/bin/bash

DEVELOPMENT="--features development"
BENCHMARK="--features runtime-benchmarks"
RELEASE=""
clear
if $1 -eq 1
then
 echo "The password is correct."
cargo build --release $DEVELOPMENT
elif $1 -eq 2
then
cargo build --release $BENCHMARK
else 
cargo build --release
fi