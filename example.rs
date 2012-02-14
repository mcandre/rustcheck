use rustcheck;
use std;

fn main() {
	std::io::println(#fmt("Random bool: %b", rustcheck::gen_bool()));
	std::io::println(#fmt("Random int: %d", rustcheck::gen_int()));
	std::io::println(#fmt("Random float: %f", rustcheck::gen_float()));
	std::io::println(#fmt("Random byte: %u", rustcheck::gen_byte()));
}