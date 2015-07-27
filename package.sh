#!/bin/bash

fpm -s dir -t $1 --name asciiauthor --force --version 0.1.0 target/release/asciiauthor=/usr/bin/asciiauthor
