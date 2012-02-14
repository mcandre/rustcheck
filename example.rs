use rustcheck;
use std;

fn main() {
	std::io::println(#fmt("Random bool: %b", rustcheck::gen_bool()));
	std::io::println(#fmt("Random int: %d", rustcheck::gen_int()));
}