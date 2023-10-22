#!/bin/bash


link=`playerctl metadata mpris:artUrl`
link_without_https="${link:24}"
format=".png"
path="$PWD/.pictures/"
final_string="${path}${link_without_https}"
final_string_with_format="${path}${link_without_https}${format}"

# download the picture with wget + spotify api album link
if [ -e "$final_string_with_format" ]; then 
  echo "Picture Already Exist. Skipping..."
else 
  wget --quiet -P "$path" "$link"
fi

# add ".png" on the end of the downloaded picture
if [ -e "$final_string" ]; then
mv "$final_string" "$final_string_with_format"
fi

# delete every picture that is not of the current playing music
if [ -e "$final_string_with_format" ]; then
find "$path" -type f ! -wholename "$final_string_with_format" -exec rm {} \;
fi
