FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=rootpassword
ENV MYSQL_DATABASE=checkit

COPY ./createtable.sql /docker-entrypoint-initdb.d/