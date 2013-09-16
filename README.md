# rustcheck - a Rust port of the QuickCheck unit test framework

# HOMEPAGE

http://www.yellosoft.us/quickcheck

# EXAMPLE

    $ make
    $ ./example
    Random bool: true
    Random int: 1864093597
    Random float: 0.057614
    Random byte: 33
    Random char: e
    Random str: 5E7
    >XpOa^
    pr`zA5pee<0P6d]>LD",

# LICENSE

FreeBSD

# REQUIREMENTS

* [rust](http://www.rust-lang.org/) 0.8+

## Optional

* [Ruby](https://www.ruby-lang.org/) 1.9+
* [Guard](http://guardgem.org/) 1.8.2+

Use `bundle` to install Guard.

# DEVELOPMENT

Start Guard in a shell, and it will automatically run unit tests when the source code changes:

    $ guard
    ...
