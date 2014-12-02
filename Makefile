all: example

example: example.rs rustcheck.rs
	rustc --crate-type=lib rustcheck.rs
	rustc example.rs -L .

lili:
	bundle exec lili .

lint: lili

clean:
	-rm example
	-rm -rf *.dylib
	-rm -rf *.dSYM
