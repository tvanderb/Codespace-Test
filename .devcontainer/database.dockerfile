FROM postgres:14.1

COPY ./migration.sql /docker-entrypoint-initdb.d/