#!/bin/bash 

BUILD_PATH=$PWD/target/release
#MAKE SURE THAT THIS UNUSED FOLDERS HAVE THE SAME START PART PATH OF THE BUILD_PATH
UNUSED_FOLDER_1=$PWD/target/release/examples
UNUSED_FOLDER_2=$PWD/target/release/incremental 
UI_PATH=$PWD/ui
FONT_PATH=$PWD/fonts

#MAKE SURE TO BE IN THE SAME FOLDER THAT THIS SCRIPT OR THE SCRIPT WILL FAIL

#FOR THIS SCRIPT BE SURE THAT IS IN THE CORRECT FOLDER HE WILL CHECK IF ONE OCULT FILE THAT I CREATE EXIST TO CONTINUE TO EXECUTE
if [ -e "$PWD/.rust-media-player-indentify" ]; then

if [ -d $BUILD_PATH ]; then
while true; do
	read -p "This App Already Has A Release Build. Overwrite It? (Y or N) : " answer

    case $answer in
        [Yy]*)
	    rm -rf $BUILD_PATH
	    cargo build --release --target-dir $BUILD_PATH
	    while [[ ! -e "$BUILD_PATH" ]]; do 
	    	sleep 1
	    done
	    if [ -e "$BUILD_PATH" ]; then
		if [ -e "$UNUSED_FOLDER_1" ] && [ -e "$UNUSED_FOLDER_2" ]; then
	    		rm -rf $UNUSED_FOLDER_1 $UNUSED_FOLDER_2
		fi

		if [ -e "$FONT_PATH" ]; then
	    	cp -rf $FONT_PATH $BUILD_PATH
		else
			echo "ERROR: FONT_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The FONT_PATh String Wrong."
		fi

		if [ -e "$UI_PATH" ]; then
	    	cp -rf $UI_PATH $BUILD_PATH
		else
			echo "ERROR: UI_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The UI_PATH String Wrong."
		fi
	    fi
            break
            ;;
        [Nn]*)
	    echo "Ok... Not Building."
            break
            ;;
        *)
            echo "Invalid input. Please enter either 'Y' or 'N'."
            ;;
    esac
done
else 
		cargo build --release --target-dir $BUILD_PATH
		while [[ ! -e "$BUILD_PATH" ]]; do 
			sleep 1
		done

		if [ -e "$UNUSED_FOLDER_1" ] && [ -e "$UNUSED_FOLDER_2" ]; then
		    	rm -rf $UNUSED_FOLDER_1 $UNUSED_FOLDER_2
		fi

		if [ -e "$FONT_PATH" ]; then
	    	cp -rf $FONT_PATH $BUILD_PATH
		else
			echo "ERROR: FONT_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The FONT_PATh String Wrong."
		fi

		if [ -e "$UI_PATH" ]; then
	    	cp -rf $UI_PATH $BUILD_PATH
		else
			echo "ERROR: UI_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The UI_PATH String Wrong."
		fi

fi

else
	echo "ERROR: You Are in The Wrong Path!. Make Sure To Be In The Same Folder Of This Scripts 'build.sh'"
fi
