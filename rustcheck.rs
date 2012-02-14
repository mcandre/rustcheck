#[link(name = "rustcheck", vers = "0.0.1")];

use std;

fn gen_bool() -> bool {
	ret (std::rand::mk_rng().next_float() * 1.0f) > 0.5f;
}

fn gen_int() -> int {
	ret std::rand::mk_rng().next() as int;
}

fn gen_float() -> float {
	ret std::rand::mk_rng().next_float();
}

fn gen_byte() -> uint {
	ret (std::rand::mk_rng().next() as int % 256) as uint;
}

fn gen_char() -> char {
	ret (std::rand::mk_rng().next() as int % 128) as char;
}