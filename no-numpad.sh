#!/bin/sh

for i in 90 87 88 89 83 84 85 79 80 81 86; do
  xmodmap -e "keycode $i = "
done
