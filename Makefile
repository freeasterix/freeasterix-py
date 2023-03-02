all:
	cargo build --release
	cp ./target/release/libfreeasterix_py.so freeasterix.so

test: all do-test

do-test:
	./test.py
