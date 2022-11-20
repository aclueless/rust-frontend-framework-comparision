#!/bin/bash
set -e

TODO=todomvc

declare -A sizes

function build_todo() {
    cd $TODO
    for x in *; do
        case $x in
            todomvc_shared | spair_shared | README.md)
                echo "Ignored $x"
                ;;
            *)
                cd $x
                trunk build --release --filehash=false
                echo ${sizes[$x]}
                sizes[$x]="${sizes[$x]} | $(stat -c%s ./dist/${TODO}_${x}_bg.wasm)"
                echo ${sizes[$x]}
                cd ../
                ;;
        esac
    done
    cd ..
}

build_todo

echo ${sizes[*]}

for key in "${!sizes[@]}"; do
    echo "$key ${sizes[$key]}";
done