#[link(name = "rustcheck", vers = "0.0.1")];

extern mod std;

use std::rand::*;
use std::str::*;
//use std::vec::*;


fn gen_bool() -> bool {
	return (std::rand::random::<float>()) > 0.5f;
}

fn gen_int() -> int {
	return std::rand::random::<int>();
}

fn gen_float() -> float {
	return std::rand::random::<float>();
}

fn gen_byte() -> uint {
	return (std::rand::random::<uint>() % 256);
}

fn gen_char() -> char {
	return (std::rand::random::<u8>() % 128) as char;
}

fn gen_vec<T: Clone>(gen : &fn() -> T, len : uint) -> ~[T] {
	if len < 1u {
		return ~[];
	}
	else {
		return ~[gen()] + gen_vec(gen, len - 1u);
	}
}

fn gen_str() -> ~str {
	let len : uint = (gen_int() % 100) as uint;
	return std::str::from_chars(gen_vec(gen_char, len));
}


fn for_all<T>(property : &fn(T) -> bool, gens : &[&fn()]) {
//    let test_case : T = gens.iter().map();
    // ...let test_case : T = vec::map(gens, { |g| ret g(); });
}
