all: prep
	(cd src/azure; rustc lib.rs  -L ../../target/x86_64-unknown-linux-gnu/lib  --out-dir ../../target/x86_64-unknown-linux-gnu/lib)
	(cd src;       rustc main.rs -L    ../target/x86_64-unknown-linux-gnu/lib  -o ../bin/igneous)

prepare:
	(cd third_party; git clone https://github.com/sfackler/rust-openssl.git; rm -rf rust-openssl/.git; git clone https://github.com/chris-morgan/rust-http.git; rm -rf rust-http/.git;)

prep:
	mkdir -p bin target/x86_64-unknown-linux-gnu/lib

test: all
	(cd src/azure; rustc test.rs -L ../../target/x86_64-unknown-linux-gnu/lib  --test  -o ../../bin/azure_test)

run: all
	./bin/igneous

runtest: test
	./bin/azure_test

deps: rust-openssl rust-http

rust-openssl: prep
	(cd third_party/rust-openssl;   make clean; ./configure; make -j1;)
	ln -s `pwd`/third_party/rust-openssl/build/*   `pwd`/target/x86_64-unknown-linux-gnu/lib/

rust-http: prep
	(cd third_party/rust-http;      make clean; ./configure; make)
	ln -s `pwd`/third_party/rust-http/build/*   `pwd`/target/x86_64-unknown-linux-gnu/lib/

clean:
	rm -rf bin/ target/