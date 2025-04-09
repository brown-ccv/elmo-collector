#!/usr/bin/env bash

source ~/.bashrc

module load rust

cd /oscar/data/ccvstaff/elmo-collector 

cargo run --release
