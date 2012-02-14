use rustcheck;
use std;

fn main() {
	std::io::println(#fmt("Random bool: %b", rustcheck::gen_bool()));
}