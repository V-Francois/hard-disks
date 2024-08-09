#!/bin/bash

mkdir -p results

for pressure in $(seq 6 -0.4 2)
do
    echo $pressure
    sed "s/PRESSURE/$pressure/" config-npt.yaml > conf.yaml
    cargo run -- conf.yaml
    mv results.yaml results/results_npt_$pressure.yaml
done

for density in $(seq 0.6 -0.05 0.4)
do
    echo $density
    sed "s/DENSITY/$density/" config-nvt.yaml > conf.yaml
    cargo run -- conf.yaml
    mv results.yaml results/results_nvt_$density.yaml
done

for density in $(seq 0.695 0.005 0.72)
do
    echo $density
    sed "s/DENSITY/$density/" config-nvt.yaml > conf.yaml
    cargo run -- conf.yaml
    mv results.yaml results/results_nvt_$density.yaml
done
