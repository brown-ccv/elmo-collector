#!/usr/bin/env bash

source secrets/db_secrets.sh

psql --host ${DB_HOST} --username postgres --dbname postgres -c "DROP DATABASE IF EXISTS elmo;"
psql --host ${DB_HOST} --username postgres --dbname postgres -c "CREATE DATABASE elmo;"
psql --host ${DB_HOST} --username postgres --dbname elmo -c "CREATE SCHEMA oscar;"