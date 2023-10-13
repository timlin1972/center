#!/bin/zsh

progs=(
    center
    plugin_wol
    plugin_sys_stat
)

for prog in ${progs[@]}
do
    killall $prog
done

cargo build

for prog in ${progs[@]}
do
    cargo run --bin $prog&
    sleep 1
done
