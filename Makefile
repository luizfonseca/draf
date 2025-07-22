bin:
	@touch ./bins/testfile
	cargo run --release --bin draf -- -o ./bins/testfile $(testfile)
	./bins/testfile

test:
	cargo run --bin test_runner --
