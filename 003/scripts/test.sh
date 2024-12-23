#!/bin/bash

for i in {200000..201000}; do curl -S localhost:4444/calc/$i; echo; done
