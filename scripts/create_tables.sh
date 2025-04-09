#!/usr/bin/env bash

source secrets/db_secrets.sh

psql --host ${DB_HOST} --username postgres --dbname elmo -f sql/create_tables.sql

