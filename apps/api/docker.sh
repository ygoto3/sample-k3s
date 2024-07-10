#!/bin/sh

set -e

if [ $# -ne 1 ]; then
  echo "Usage: $0 <tag>"
  exit 1
fi

docker build --platform linux/amd64 -t $1 .
docker push $1
