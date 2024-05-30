#!/bin/bash
FILIO_VERSION="1.1"
FILIO_PATH="$HOME/.filio"
ALIAS_COMMAND="alias filio=$FILIO_PATH/bin/filio"


# Needed Sudo for Version 1.0 
# if [ $EUID != 0 ]; then
#   echo -e "Please run the command file using 'sudo'
# to see what is inside install.sh visit
# https://github.com/Masoom-Wahid/filio/install.sh"
#   exit 1
# fi

echo "Installing Filio....."

if ! [ -d "$FILIO_PATH" ]; then
  mkdir -p "$FILIO_PATH/data" 
  mkdir -p "$FILIO_PATH/logs"
  mkdir -p "$FILIO_PATH/bin" 
  touch "$FILIO_PATH/data/filio.json"
  touch "$FILIO_PATH/logs/out.log"
fi


curl -L https://github.com/Masoom-Wahid/filio/releases/download/v$FILIO_VERSION/filio > $FILIO_PATH/bin/filio && chmod +x $FILIO_PATH/bin/filio 

echo $ALIAS_COMMAND >> $HOME/.bashrc 2>/dev/null
echo $ALIAS_COMMAND >> $HOME/.zshrc 2>/dev/null

echo "Filio installed successfully!"
echo "run 'filio help' for additional help on commands"
