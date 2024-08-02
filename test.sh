#!/bin/bash

export BEARER_TOKEN=master69xxx

/usr/bin/ab \
  -n 10000 \
  -c 10 \
  -p ./postfile \
  -T "application/json" \
  -m POST \
  -H "Authorization: Bearer $BEARER_TOKEN" \
  http://localhost:8080/check

