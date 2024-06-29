#!/bin/sh

set -ex

wasm-pack build --target web
mkdir -p www
rm -f www/*
cp index.html www/
cp js/*.js www/
cp pkg/*.js www/
cp pkg/*.wasm www/
cd www
python3 -m http.server
