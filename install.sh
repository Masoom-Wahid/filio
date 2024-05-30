#!/bin/bash
FILIO_VERSION="1.1"
USERNAME=$(whoami)
FILIO_PATH="/home/$USERNAME/.filio"

if [ $EUID != 0 ]; then
  echo -e "Please run the command file using 'sudo'
to see what is inside install.sh visit
https://github.com/Masoom-Wahid/filio/install.sh"
  exit 1
fi

echo "Installing Filio....."

if ! [ -d "$FILIO_PATH" ]; then
  mkdir -p "$FILIO_PATH/data" 
  mkdir -p "$FILIO_PATH/logs"
  touch "$FILIO_PATH/data/filio.json"
  touch "$FILIO_PATH/logs/out.log"
fi


curl -L https://github.com/Masoom-Wahid/filio/releases/download/v$FILIO_VERSION/filio > /usr/bin/filio && chmod +x /usr/bin/filio

echo "Filio installed successfully!"
echo "run 'filio help' for additional help on commands"
