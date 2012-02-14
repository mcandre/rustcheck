all: example

example: example.rs rustcheck.rs
	rustc --lib rustcheck.rs
	rustc example.rs -L .

clean:
	-rm example
	-rm -rf *.dylib
	-rm -rf *.dSYM