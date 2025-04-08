#!/usr/bin/env bash

sqlite3 data/oscar.db -csv -header "SELECT * FROM cpu;" > data/cpu.csv
sqlite3 data/oscar.db -csv -header "SELECT * FROM gpu;" > data/gpu.csv
sqlite3 data/oscar.db -csv -header "SELECT * FROM power_save;" > data/power_save.csv
sqlite3 data/oscar.db -csv -header "SELECT * FROM power_save_cpu;" > data/power_save_cpu.csv
sqlite3 data/oscar.db -csv -header "SELECT * FROM power_save_gpu;" > data/power_save_gpu.csv

