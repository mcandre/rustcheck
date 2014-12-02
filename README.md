# rustcheck - a Rust port of the QuickCheck unit test framework

# HOMEPAGE

http://www.yellosoft.us/quickcheck

# EXAMPLE

```
$ make
mkdir -p bin/
rustc --crate-type=lib rustcheck.rs
rustc -o bin/example example.rs -L .
bin/example
task '<main>' failed at 'assertion failed: for_all(prop_even, x)', example.rs:36
make: [test] Error 101 (ignored)
```

# LICENSE

FreeBSD

# REQUIREMENTS

* [rust](http://www.rust-lang.org/) 0.11

## Optional

* [Ruby](https://www.ruby-lang.org/) 1.9+
* [Guard](http://guardgem.org/) 1.8.2+
* [aspelllint](https://github.com/mcandre/aspelllint)

Install Guard and aspelllint:

    $ bundle

# DEVELOPMENT

## Spell Check

    $ aspelllint
    ...

## Local CI

Start Guard in a shell, and it will automatically run unit tests when the source code changes:

    $ guard
    ...
