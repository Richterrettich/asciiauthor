#!/bin/bash

fpm -s dir -t $1 --name asciiauthor --version 0.1.0 target/release/asciiauthor=/usr/bin/asciiauthor
