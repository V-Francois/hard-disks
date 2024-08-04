#!/bin/bash

mkdir -p results

for pressure in $(seq 9 -0.4 3)
do
    echo $pressure
    sed "s/PRESSURE/$pressure/" config.yaml > conf.yaml
    cargo run -- conf.yaml
    mv thermo.csv results/thermo_$pressure.csv
done
