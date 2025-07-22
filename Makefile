bin:
	@touch ./bins/testfile
	cargo run --release --bin draf -- -o ./bins/testfile $(testfile)
	./bins/testfile

build:
	cargo build --release --bin draf

test:
	cargo run --bin test_runner --
