#[link(name = "rustcheck", vers = "0.0.1")];

extern mod std;

use std::rand::random;
use std::str::from_chars;


pub fn gen_bool() -> bool {
	return (std::rand::random::<f32>()) > 0.5f32;
}

pub fn gen_int() -> int {
	return std::rand::random::<int>();
}

pub fn gen_float() -> f32 {
	return std::rand::random::<f32>();
}

pub fn gen_byte() -> uint {
	return (std::rand::random::<uint>() % 256);
}

pub fn gen_char() -> char {
	return (std::rand::random::<u8>() % 128) as char;
}

pub fn gen_vec<T: Clone>(gen : (|| -> T), len : uint) -> ~[T] {
	if len < 1u {
		return ~[];
	}
	else {
		return ~[gen()] + gen_vec(gen, len - 1u);
	}
}

pub fn gen_str() -> ~str {
	let len : uint = (gen_int() % 100) as uint;
	return std::str::from_chars(gen_vec(gen_char, len));
}


pub fn for_all<T>(property : |T| -> bool, gens : &[|| -> T]) -> bool {
    let mut result = true;

    for g in gens.iter() {
        let v: T = (*g)();
        result = result & property(v);
    }

    return result;
}
