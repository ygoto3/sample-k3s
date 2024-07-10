#!/bin/sh

docker run -it -d -p 17170:17170 -p 3890:3890 \
  -e LLDAP_VERBOSE=true \
  -e LLDAP_JWT_SECRET=$(openssl rand 256 | base64) \
  -e LLDAP_KEY_SEED=$(openssl rand 256 | base64) \
  -e LLDAP_LDAP_BASE_DN=${LLDAP_LDAP_BASE_DN} # dc=ldap,dc=example,dc=com \
  --name ldap \
  lldap/lldap:stable
