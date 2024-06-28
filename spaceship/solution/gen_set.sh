#!/bin/bash

name=$1
shift 1

if [[ $name == "" ]]; then
	echo "Usage $0 name seeds.txt"
	exit
fi

if [[ $1 == "" || $1 == -* ]]; then
	seeds="seeds.txt"
else
	seeds=$(readlink -f $1)
	shift 1
fi


mkdir $name || exit 1
pushd ../tools/ > /dev/null
rm -rf in
cargo run --release --bin gen $seeds -v "$@" > in.csv
# cargo run --release --bin gen $seeds "$@"
# awk 'BEGIN {print "file,seed"} {printf "%04d,%s\n", NR-1, $0}' ../tools/seeds.txt > in.csv
popd > /dev/null
mv ../tools/in $name/
mv ../tools/in.csv $name/
