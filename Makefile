.DEFAULT_GOAL := package


test:
	cd src/clientapp; npm install; npm run build
	rm -r static_files
	mv src/clientapp/build static_files
	cargo run

package:
	rm -rf build
	rm -rf build-pi
	mkdir build
	cd src/clientapp; npm install; npm run build
	rm -rf static_files
	mv src/clientapp/build static_files
	cross build --release
	cross strip
	mv target/release/alexbox build/alexbox
	cp -r static_files build/static_files
	cp config.xml build/config.xml
	cp .env build/.env
	cp AUTHORS build/AUTHORS
	cp LICENSE build/LICENSE

package-pi:
	rm -rf build
	rm -rf build-pi
	mkdir build-pi
	cd src/clientapp; npm install; npm run build
	rm -rf static_files
	mv src/clientapp/build static_files
	cross build --target armv7-unknown-linux-gnueabihf --release
	cross strip --target armv7-unknown-linux-gnueabihf
	mv target/armv7-unknown-linux-gnueabihf/release/alexbox build-pi/alexbox
	cp -r static_files build-pi/static_files
	cp config.xml build-pi/config.xml
	cp .env build-pi/.env
	cp AUTHORS build-pi/AUTHORS
	cp LICENSE build-pi/LICENSE


install:
	mv -r build/* /opt/etc/alexbox/
