
extern mod rustcheck;

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

	assert!(rustcheck::for_all(prop_even, &[a]));
	assert!(rustcheck::for_all(prop_even, &[b]));
}
