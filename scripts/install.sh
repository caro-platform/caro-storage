#!/bin/bash

BIN_DIR="/usr/local/bin/"
SETTINGS_DIR="/etc/krossbar/settings/"

BUID_TYPE=debug

installBin() {
    local FILE=$1
    echo "Copying binary: '../target/${BUID_TYPE}/${FILE}' into '${BIN_DIR}'"

    sudo cp -fu "../target/${BUID_TYPE}/${FILE}" "${BIN_DIR}"
}

for i in "$@"; do
    case $i in
        -b|--bin)
            installBin $2
            shift
            shift
            ;;
        --release)
            BUID_TYPE=release
            shift
            ;;
        -*|--*)
            echo "Unknown option $i"
            exit 1
            ;;
        *)
        ;;
    esac
done

sudo mkdir -p "${SETTINGS_DIR}"
