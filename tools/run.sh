cargo b --release
cargo objcopy --release -- -O ihex tools/rp2040js/solarreader.hex
cd tools/rp2040js
npm install
npm start
cd -
