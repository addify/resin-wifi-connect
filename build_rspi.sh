scripts/local-build.sh arm-unknown-linux-gnueabihf rpi
cp target/arm-unknown-linux-gnueabihf/release/wifi-connect .
tar -czvf addify-wifi-connect.tar.gz ./wifi-connect ./ui