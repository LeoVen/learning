#!/bin/bash

for i in {1..1000}; do sleep 1 && curl -s localhost:5050/greet/world | jq -j ".copy" ; done
