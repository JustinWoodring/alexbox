# Alexbox
![Screenshot](https://github.com/Booglejr/alexbox/raw/assets/images/alexbox.png)
Alexbox is a scheduling tool for mpv that lets schedule media to be played at certain times of the day. 

**The python script that connects with mpv has not been uploaded yet.**

## Running

You'll need docker, rust, npm, cross, and cargo-strip.

1. So get docker somehow.

2. Install rust and npm.

3. Then run
```
cargo install cargo-strip
cargo install cross
```

4. Now run the following to actually run the server.
```
git clone https://github.com/Booglejr/alexbox
make test
```

## Installing
1. Do every step in Running except replace `make test` with `make package` or `make package-pi`

2. All the files you need to install somewhere will be placed into `build` or `build-pi`

3. Take these files and put them somewhere like `/etc/opt/alexbox`

## Documentation

For more documentation please see the wiki, [Alexbox Wiki](https://github.com/Booglejr/alexbox/wiki)
