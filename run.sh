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

cargo fmt
cargo clippy

for prog in ${progs[@]}
do
    cargo build --bin $prog
    cargo run --bin $prog&
    sleep 1
done
