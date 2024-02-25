#!/bin/bash
if [ $# -ne 1 ]; then
    echo "USAGE: $0 <online|local>"
    echo " e.g.: $0 online"
    exit 1
fi
MODE=$1

copy_conf_files() {
    cp ./$MODE/my1.Cargo.toml "../my1/Cargo.toml"
    cp ./$MODE/my2.Cargo.toml "../my2/Cargo.toml"
    cp ./$MODE/my3.Cargo.toml "../my3/Cargo.toml"
    cp ./$MODE/myexe.Cargo.toml "../myexe/Cargo.toml"
}

# do some operations
if [ "$MODE" = "online" ]; then
    echo "copy online cargo project files..."
    copy_conf_files
elif [ "$MODE" = "local" ]; then
    echo "copy local cargo project files..."
    copy_conf_files
else
    echo "not supported mode: $MODE, input <online|local>"
fi