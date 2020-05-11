#!/bin/sh

if [[ $# -eq 1 && $1 == "uninstall" ]]; then

    # Uninstall
    rm /usr/lib/nautilus/extensions-3.0/libnautilus-custom-terminal.so

    echo "Uninstalled"

else

    # Install
    cp target/release/libnautilus_custom_terminal.so /usr/lib/nautilus/extensions-3.0/libnautilus-custom-terminal.so

    echo "Installed"

fi
