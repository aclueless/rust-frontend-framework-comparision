#!/bin/bash
set -ex

TODO=./todomvc

for x in $TODO/*; do
    case $x in
    	$TODO/todomvc_shared | $TODO/spair_shared | $TODO/README.md)
    	    echo "Ignored $x"
    	    ;;
    	*)
    	    cd $x
    	    trunk build --release
    	    cd ../../
    	    ;;

    esac
done
