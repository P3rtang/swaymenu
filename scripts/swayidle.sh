#!/bin/bash
TIME_MOD_SMALL=30
TIME_MOD_BIG=60
swayidle -w timeout $TIME_MOD_SMALL 'swaylock -f -i $HOME/Pictures/wallpapers/factorio.jpg && swaymsg "output * dpms off"' resume 'swaymsg "output * dpms on"' timeout $TIME_MOD_BIG 'systemctl suspend' before-sleep 'swaylock -f -i $HOME/Pictures/wallpapers/factorio.jpg'
