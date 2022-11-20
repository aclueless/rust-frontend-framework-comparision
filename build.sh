#!/bin/bash
set -e

TODO=todomvc

declare -A sizes

function build_todo() {
    cd $TODO
    for x in *; do
        case $x in
            todomvc_shared | spair_shared | README.md | results.md)
                echo "ignored $x"
                ;;
            *)
                cd $x
                trunk build --release --filehash=false
                sizes[$x]="${sizes[$x]} | $(stat -c%s ./dist/${TODO}_${x}_bg.wasm)"
                cd ../
                ;;
        esac
    done
    cd ..
}

function output_sizes() {
    declare -a names
    for key in "${!sizes[@]}"; do
    	names+=($key);
    done
    sorted_names=($(echo ${names[*]}| tr " " "\n" | sort -n))
    echo "# Build results"
    echo "\`\`\`"
    echo $(rustc -V)
    echo $(trunk -V)
    echo "\`\`\`"
    echo "| Implementations | opt-level = 3 | opt-level = 's' | opt-level = 'z' |"
    echo "|-----------------|---------------------|-----------------|-----------------|"
    for name in ${sorted_names[@]}; do
        echo "| $name ${sizes[$name]} |";
    done
    echo ""
    echo "This file is generated automatically by a script."
    echo ""
    echo "Any changes made to this file will be overwriten by a new generated file."
}

sed '4s/.*/opt-level = 3/' Base.toml > Cargo.toml
build_todo
sed '4s/.*/opt-level = "s"/' Base.toml > Cargo.toml
build_todo
sed '4s/.*/opt-level = "z"/' Base.toml > Cargo.toml
build_todo

output_sizes > ./$TODO/results.md