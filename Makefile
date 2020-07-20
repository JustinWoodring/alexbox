.DEFAULT_GOAL := package


test:
	cd src/clientapp; npm install; npm run build
	rm -r static_files
	mv src/clientapp/build static_files
	cargo run

package:
	rm -rf build
	mkdir build
	cd src/clientapp; npm install; npm run build
	rm -rf static_files
	mv src/clientapp/build static_files
	cargo build --release
	mv target/release/alexbox build/alexbox
	cp -r static_files build/static_files
	cp config.xml build/config.xml
	cp .env build/.env
	cp planner.db build/planner.db
	cp AUTHORS build/AUTHORS
	cp LICENSE build/LICENSE

install:
	mv -r build/* /opt/etc/alexbox/