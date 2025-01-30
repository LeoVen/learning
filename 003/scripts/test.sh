#!/bin/bash

for i in {1..500000}; do curl -S "localhost:4444/calc/${i}"; echo; done
