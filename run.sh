#!/bin/zsh

progs=(
    center
    plugin_wol
    plugin_sys_stat
    plugin_shutdown
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
    sleep 5
done
