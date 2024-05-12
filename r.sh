echo "Compiling"
cargo build --release

echo "Stoping Systemctl"
sudo systemctl stop filio.service

echo "Copying"
sudo cp "./target/release/filio" "/bin/filio"


echo "Starting Systemctl"
sudo systemctl start filio.service
