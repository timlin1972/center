#!/bin/zsh

progs=(
    center
    plugin_wol
    plugin_sys_stat
    plugin_shutdown
    plugin_screen
    plugin_log
)

for prog in ${progs[@]}
do
    killall $prog
done

cargo fmt
cargo clippy
cargo build --release

for prog in ${progs[@]}
do
    ./target/release/$prog&
    sleep 1
done
