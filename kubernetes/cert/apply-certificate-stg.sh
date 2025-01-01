#!/bin/sh

cat certificate-stg.yaml | envsubst | kubectl apply -f -