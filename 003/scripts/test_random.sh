#!/bin/bash

for i in {0..1000}; do curl -S "localhost:4444/calc/$(shuf -i 1-100000 -n 1)"; echo; done
