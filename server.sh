#!/bin/zsh

progs=(
    center
    plugin_wol
    plugin_sys_stat
    plugin_shutdown
    plugin_screen
)

for prog in ${progs[@]}
do
    echo "kill $prog"
    killall $prog
done

for prog in ${progs[@]}
do
    echo "run $prog"
    ./target/debug/$prog&
    sleep 20
done
