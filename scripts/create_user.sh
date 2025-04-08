#!/usr/bin/env bash

source secrets/db_secrets.sh

psql --host ${DB_HOST} --username postgres --dbname postgres -c "CREATE USER elmo_svc WITH PASSWORD '${DB_PASSWORD}';"



