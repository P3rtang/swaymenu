#!/bin/bash
export LIBVIRT_DEFAULT_URI="qemu:///system"
virsh --connect=qemu:///system start win11