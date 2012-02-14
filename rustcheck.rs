#[link(name = "rustcheck", vers = "0.0.1")];

use std;

fn gen_bool() -> bool {
	ret (std::rand::mk_rng().next_float() * 1.0f) > 0.5f;
}