#!/bin/bash 

BUILD_PATH=$PWD/target/
PATH_TO_COPY=$PWD/target/debug/
#MAKE SURE THAT THIS UNUSED FOLDERS HAVE THE SAME START PART PATH OF THE BUILD_PATH
UNUSED_FOLDER_1=$PWD/target/debug/examples
UNUSED_FOLDER_2=$PWD/target/debug/incremental 
UI_PATH=$PWD/ui
FONT_PATH=$PWD/fonts

#MAKE SURE TO BE IN THE SAME FOLDER THAT THIS SCRIPT OR THE SCRIPT WILL FAIL
if [ -e "$PWD/Cargo.toml" ]; then

if [ -d $BUILD_PATH ]; then
while true; do
	read -p "This App Already Has A Release Build. Overwrite It? (Y or N) : " answer

    case $answer in
        [Yy]*)
	    #rm -rf $BUILD_PATH
	    #cargo build --release --target-dir $BUILD_PATH
	    cargo build --target-dir $BUILD_PATH
	    while [[ ! -d "$BUILD_PATH" ]]; do 
	    	sleep 1
	    done
	    if [ -d "$BUILD_PATH" ]; then
		if [ -d "$UNUSED_FOLDER_1" ] && [ -d "$UNUSED_FOLDER_2" ]; then
	    		rm -rf $UNUSED_FOLDER_1 $UNUSED_FOLDER_2
		fi

		if [ -d "$FONT_PATH" ]; then
	    		cp -rf $FONT_PATH $PATH_TO_COPY
		else
			echo "ERROR: FONT_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The FONT_PATh String Wrong."
		fi

		if [ -d "$UI_PATH" ]; then
	    		cp -rf $UI_PATH $PATH_TO_COPY
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
		while [[ ! -d "$BUILD_PATH" ]]; do 
			sleep 1
		done

		if [ -d "$UNUSED_FOLDER_1" ] && [ -d "$UNUSED_FOLDER_2" ]; then
		    	rm -rf $UNUSED_FOLDER_1 $UNUSED_FOLDER_2
		fi

		if [ -d "$FONT_PATH" ]; then
	    		cp -rf $FONT_PATH $PATH_TO_COPY
		else
			echo "ERROR: FONT_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The FONT_PATh String Wrong."
		fi

		if [ -d "$UI_PATH" ]; then
	    		cp -rf $UI_PATH $PATH_TO_COPY
		else
			echo "ERROR: UI_PATH NOT FOUND!. Maybe The Folder Got Deleted Or This Shell Script Has The UI_PATH String Wrong."
		fi

fi

else
	echo "ERROR: You Are in The Wrong Path!. Make Sure To Be In The Same Folder Of This Scripts 'build.sh'"
fi
