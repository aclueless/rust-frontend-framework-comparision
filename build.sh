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
                #echo ${sizes[$x]}
                sizes[$x]="${sizes[$x]} | $(stat -c%s ./dist/${TODO}_${x}_bg.wasm)"
                #echo ${sizes[$x]}
                cd ../
                ;;
        esac
    done
    cd ..
}

sed '4s/.*/opt-level = 3/' Base.toml > Cargo.toml
build_todo
sed '4s/.*/opt-level = "s"/' Base.toml > Cargo.toml
build_todo
sed '4s/.*/opt-level = "z"/' Base.toml > Cargo.toml
build_todo

function output_sizes() {
    echo "```"
    echo $(rustc -V)
    echo $(trunk -V)
    echo "```"
    echo "| Implementations | opt-level = 3 | opt-level = 's' | opt-level = 'z' |"
    echo "|-----------------|---------------------|-----------------|-----------------|"
    for key in "${!sizes[@]}"; do
        echo "| $key ${sizes[$key]} |";
    done
    echo ""
    echo "This file is generated automatically by a script."
    echo "Any changes made to this file will be lost by a new generated file."
}

output_sizes > ./$TODO/results.md