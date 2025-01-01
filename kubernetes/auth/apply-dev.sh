#!/bin/sh

kustomize build --load-restrictor LoadRestrictionsNone dev | envsubst | kubectl apply -f -
