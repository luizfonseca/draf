bin:
	@rm ./bins/testfile
	@cargo run --release -- -o ./bins/testfile $(testfile)
	./bins/testfile
