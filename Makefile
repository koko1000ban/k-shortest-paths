
all:
	mkdir -p ./lib
	rustc --lib ./src/network.rc --out-dir ./lib

test: all
	export RUST_THREADS=1
	mkdir -p ./build
	rustc --test ./src/network.rc --out-dir ./build
	rustc --test -L ./lib ./test/simple.rs --out-dir ./build
	find ./build -perm -u+x -type f -exec {} \;

clean:
	rm -rf ./lib
	rm -rf ./build
