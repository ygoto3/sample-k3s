#!/bin/sh

cat certificate.yaml | envsubst | kubectl apply -f -