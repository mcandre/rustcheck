BIN=bin/example

all: test

test: $(BIN)
	-$(BIN)

$(BIN): example.rs rustcheck.rs
	mkdir -p bin/
	rustc --crate-type=lib rustcheck.rs
	rustc -o $(BIN) example.rs -L .

lili:
	bundle exec lili .

lint: lili

clean:
	-rm -rf bin/
	-rm -rf *.rlib
	-rm -rf *.dylib
	-rm -rf *.dSYM
