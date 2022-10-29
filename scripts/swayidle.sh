#!/bin/bash
TIME_MOD_SMALL=300
TIME_MOD_BIG=600
while getopts 'ed' OPTION; do
  case "$OPTION" in
    e)
      swayidle -w timeout $TIME_MOD_SMALL 'swaylock -f -i $HOME/Pictures/wallpapers/factorio.jpg && swaymsg "output * dpms off"' resume 'swaymsg "output * dpms on"' timeout $TIME_MOD_BIG 'systemctl suspend' before-sleep 'swaylock -f -i $HOME/Pictures/wallpapers/factorio.jpg'
      ;;
    d)
      killall swayidle
      swayidle -w before-sleep 'swaylock -f -i $HOME/Pictures/wallpapers/factorio.jpg'
      ;;
  esac
done
shift "$(($OPTIND -1))"