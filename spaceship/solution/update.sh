#!/bin/bash

cargo build --release
cd ../tools
cargo build --release
