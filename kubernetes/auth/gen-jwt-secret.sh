#!/bin/sh

node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"
