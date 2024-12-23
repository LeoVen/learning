#!/bin/bash

for i in {1..1000}; do sleep $(echo $(($RANDOM % 100))) && curl -s localhost:5050/greet/world | jq -j ".copy" ; done
