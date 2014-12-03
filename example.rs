//! Example rustcheck usage

#![feature(phase)]

#[phase(plugin, link)] extern crate rustcheck;

use rustcheck::gen_int;

// Property: All integers are even
fn prop_even(x : int) -> bool {
  // Modulo check for evenness
	return x % 2 == 0;
}

// Generate a random even integer
fn gen_even() -> int {
  // Based on rustcheck's integer generator
	let i : int = gen_int();

	if i % 2 == 0 {
    i
	}
	else {
    i + 1
	}
}

fn main() {
  let a = || gen_int();
  let b = || gen_even();

  let mut x = Vec::new();
  x.push(a);

  let mut y = Vec::new();
  y.push(b);

	for_all!(prop_even, x);
	for_all!(prop_even, y);
}
