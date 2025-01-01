#!/bin/sh

cat issuer-stg.yml | envsubst | kubectl apply -f -
cat issuer.yml | envsubst | kubectl apply -f -