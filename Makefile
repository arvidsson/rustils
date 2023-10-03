.PHONY: all

all: bin/httpcat

bin/%: %/src/main.rs
	cd $* && cargo build -r && cp target/release/$* ../bin