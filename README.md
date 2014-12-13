# rustcheck - a Rust port of the QuickCheck unit test framework

# HOMEPAGE

http://www.yellosoft.us/quickcheck

# EXAMPLE

```
$ make
mkdir -p bin/
rustc --crate-type=lib rustcheck.rs -O -W missing-doc -W non-uppercase-statics -W unnecessary-qualification -W unnecessary-typecast -W unused-result
rustc -o bin/example example.rs -L . -O -W missing-doc -W non-uppercase-statics -W unnecessary-qualification -W unnecessary-typecast -W unused-result
bin/example
task '<main>' failed at '621423509815437667', example.rs:38
make: [test] Error 101 (ignored)
```

See [example.rs](https://github.com/mcandre/rustcheck/blob/master/example.rs) for more information.

# LICENSE

FreeBSD

# REQUIREMENTS

* [rust](http://www.rust-lang.org/) 0.12

## Optional

* [Ruby](https://www.ruby-lang.org/) 1.9+
* [Guard](http://guardgem.org/) 1.8.2+
* [aspelllint](https://github.com/mcandre/aspelllint)

Install Guard and aspelllint:

```
$ bundle
```

# DEVELOPMENT

## Spell Check

```
$ aspelllint
...
```

## Local CI

Start Guard in a shell, and it will automatically run unit tests when the source code changes:

```
$ guard
...
```
