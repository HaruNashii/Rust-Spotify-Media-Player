#!/bin/bash


link=`playerctl metadata mpris:artUrl`
link_without_https="${link:24}"

holder_name="holder.jpg"
blur_name="blur.png"
format=".png"
path="$PWD/.background/"

background_without_format="${path}${link_without_https}"
background_with_format="${background_without_format}${format}"
blur_image="${path}${blur_name}"
holder_image="${path}${holder_name}"


# download the picture with wget + spotify api album link
if ! [ -e "$background_with_format" ]; then 
  wget --quiet -P "$path" "$link"
fi

# add ".png" on the end of the downloaded picture
if [ -e "$background_without_format" ]; then
    if ! [ -e "$background_with_format" ]; then
      mv "$background_without_format" "$background_with_format"
    fi
fi

# delete every picture that is not of the current playing music
if [ -e "$background_with_format" ]; then
  find "$path" -type f ! -wholename "$background_with_format" ! -wholename "$holder_image" ! -wholename "$blur_image" -exec rm {} \;
fi
