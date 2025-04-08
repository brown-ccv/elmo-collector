DROP TABLE IF EXISTS oscar.cpu;

CREATE TABLE oscar.cpu(
    id serial PRIMARY KEY,
    time timestamp,
    allocated integer,
    idle integer,
    other integer,
    total integer
);

\COPY oscar.cpu FROM 'data/cpu.csv' DELIMITER ',' CSV HEADER;
DROP TABLE IF EXISTS oscar.gpu;

CREATE TABLE oscar.gpu(
    id serial PRIMARY KEY,
    time timestamp,
    allocated integer,
    total integer
);

\COPY oscar.gpu FROM 'data/gpu.csv' DELIMITER ',' CSV HEADER;
DROP TABLE IF EXISTS oscar.power_save;

CREATE TABLE oscar.power_save(
    id serial PRIMARY KEY,
    time timestamp,
    power_save integer,
    total integer
);

\COPY oscar.power_save FROM 'data/power_save.csv' DELIMITER ',' CSV HEADER;
DROP TABLE IF EXISTS oscar.power_save_cpu;

CREATE TABLE oscar.power_save_cpu(
    id serial PRIMARY KEY,
    time timestamp,
    power_save integer,
    total integer
);

\COPY oscar.power_save_cpu FROM 'data/power_save_cpu.csv' DELIMITER ',' CSV HEADER;
DROP TABLE IF EXISTS oscar.power_save_gpu;

CREATE TABLE oscar.power_save_gpu(
    id serial PRIMARY KEY,
    time timestamp,
    power_save integer,
    total integer
);

\COPY oscar.power_save_gpu FROM 'data/power_save_gpu.csv' DELIMITER ',' CSV HEADER;
