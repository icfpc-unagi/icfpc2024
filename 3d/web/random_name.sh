#!/bin/bash

h=$(head /dev/urandom | tr -dc A-Za-z | head -c 1);
b=$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 7);
echo $h$b
