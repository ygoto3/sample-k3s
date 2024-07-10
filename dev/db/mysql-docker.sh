#!/bin/sh

docker run --name sample-k3s-mysql \
    -e MYSQL_ROOT_PASSWORD=devpassword \
    -e MYSQL_USER=mysql \
    -e MYSQL_PASSWORD=mysqldevpassword \
    -e MYSQL_DATABASE=db \
    --mount type=bind,source=$(pwd)/docker-entrypoint-initdb.d,target=/docker-entrypoint-initdb.d \
    -d -p 3306:3306 mysql:9.0.0
