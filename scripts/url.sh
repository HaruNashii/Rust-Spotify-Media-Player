#!/bin/bash

link=`playerctl metadata mpris:artUrl`
link_without_https="${link:24}"
path="$PWD/.pictures/"
format=".png"

final_string="${path}${link_without_https}${format}"

echo "$final_string"
