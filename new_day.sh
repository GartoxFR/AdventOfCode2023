#!/usr/bin/env bash

if [[ -z $1 ]]; then
    echo "No name supplied"
    exit 1
fi

cargo new $1

rm -v $1/src/main.rs
mkdir -v $1/src/bin
touch $1/input{1,2}.txt
cp -v template.rs $1/src/bin/part1.rs

cd $1
cargo add nom

