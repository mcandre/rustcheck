extern crate rustcheck;

use rustcheck::{gen_int,for_all};

fn prop_even(x : int) -> bool {
	return x % 2 == 0;
}

fn gen_even() -> int {
	let i : int = rustcheck::gen_int();

	if i % 2 == 0 {
    i
	}
	else {
    i + 1
	}
}

fn main() {
  let a = || rustcheck::gen_int();
  let b = || gen_even();

  let mut x = Vec::new();
  x.push(a);

  let mut y = Vec::new();
  y.push(b);

	assert!(rustcheck::for_all(prop_even, x));
	assert!(rustcheck::for_all(prop_even, y));
}
