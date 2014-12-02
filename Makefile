FLAGS=-O -W missing-doc -W non-uppercase-statics -W unnecessary-qualification -W unnecessary-typecast -W unused-result

BIN=bin/example

all: test

test: $(BIN)
	-$(BIN)

$(BIN): example.rs rustcheck.rs
	mkdir -p bin/
	rustc --crate-type=lib rustcheck.rs $(FLAGS)
	rustc -o $(BIN) example.rs -L . $(FLAGS)

lili:
	bundle exec lili .

lint: lili

clean:
	-rm -rf bin/
	-rm -rf *.rlib
	-rm -rf *.dylib
	-rm -rf *.dSYM
