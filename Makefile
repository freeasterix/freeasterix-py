all:
	cargo build --release
	cp ./target/release/libfreeasterix_py.so freeasterix.so

test: all do-test

do-test:
	echo 'import freeasterix; print(freeasterix.encode(dict(a=123, b=2.4)))' | python3
