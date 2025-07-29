#!/usr/bin/env bash

set -e

meson setup --reconfigure build
ninja -C build
sudo rm -rf /usr/local/share/wayfire/metadata
sudo rm -rf /usr/local/lib64/wayfire
sudo ninja -C build install
