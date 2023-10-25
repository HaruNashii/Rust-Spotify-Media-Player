#!/bin/bash

link=`playerctl metadata mpris:artUrl`
link_without_https="${link:24}"

path="$PWD/.background/"
format=".png"
holder_image="holder.jpg"

background_with_format="${path}${link_without_https}${format}"
holder_string="${path}${holder_image}${format_holder}"

if [ -e "$background_with_format" ]; then 
echo "$background_with_format"
else 
echo  "$holder_string"
fi
