#!/bin/bash

qemu-system-x86_64 -drive format=raw,file=target/x86_64-fe_os/debug/bootimage-fe_os.bin
