all:
	(cd src/azure; rustc lib.rs  -L ../../target/x86_64-unknown-linux-gnu/lib  --out-dir ../../target/x86_64-unknown-linux-gnu/lib)
	(cd src;       rustc main.rs -L    ../target/x86_64-unknown-linux-gnu/lib  -o ../bin/igneous)

test: all
	(cd src/azure; rustc test.rs -L ../../target/x86_64-unknown-linux-gnu/lib  --test  -o ../../bin/azure_test)

run: all
	./bin/main

runtest: test
	./bin/azure_test

deps: rust-openssl rust-http

rust-openssl:
	(cd third_party/rust-openssl;   make clean; ./configure; make;)

rust-http:
	(cd third_party/rust-http;      make clean; ./configure; make)

#capnproto-rust:
#	(cd third_party/capnproto-rust; make clean; ./configure; make)