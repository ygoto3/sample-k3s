#!/bin/sh

kubectl label namespace data istio-injection=enabled --overwrite
kubectl label namespace api istio-injection=enabled --overwrite
