//! Rust port of the Quickcheck unit test framework

#![crate_id(name = "rustcheck", vers = "0.0.1")]

extern crate std;

use std::rand::random;

// Generate a random boolean
pub fn gen_bool() -> bool {
  //! Treat random floats greater than 0.5 as true
	return random::<f32>() > 0.5f32;
}

// Generate a random integer
pub fn gen_int() -> int {
  //! Use built-in Rust int generator
	return random::<int>();
}

// Generate a random floating point value
pub fn gen_float() -> f32 {
  //! Use builtin Rust float generator
	return random::<f32>();
}

// Generate a random byte
pub fn gen_byte() -> uint {
  //! Force random unsigned integers into byte size
	return random::<uint>() % 256;
}

// Generate a random UTF8 character
pub fn gen_uchar() -> u8 {
  //! Use built-in Rust u8 generator
	return random::<u8>();
}

// Generate a random ASCII character
pub fn gen_char() -> char {
  //! Force random u8 into 7-bit size
	return (gen_uchar() % 128) as char;
}

// Given a generator function, generate a vector
// populated by values output from the function
pub fn gen_vec<T: Clone>(gen : (|| -> T), len : uint) -> Vec<T> {
  //! Empty vector
	if len < 1u {
		return Vec::new();
	}
	else {
    let mut v : Vec<T> = Vec::new();
    v.push(gen());

    let w : Vec<T> = gen_vec(gen, len - 1u);

    return v + w;
	}
}

// Generate a random UTF8 string
pub fn gen_string() -> String {
  //! From 0 to 99 characters
	let len : uint = (gen_int() % 100) as uint;

  let u8vec : Vec<u8> = gen_vec(gen_uchar, len);

  let u8s : &[u8] = u8vec.as_slice();

  let option_str : Option<&str> = std::str::from_utf8(u8s);

  let s : &str = option_str.unwrap();

  return s.to_string();
}

// Give a property and a collection of generators,
// test the property over random test cases
// created by calling the generator functions
pub fn for_all<T>(property : |T| -> bool, mut gens : Vec<|| -> T>) -> bool {
  //! Assume property is true

  let mut result = true;

  for _ in range(1u, 100u) {
    for g in gens.mut_iter() {
      let v : T = (*g)();

      // Check result of each test case
      result = result & property(v);
    }
  }

  return result;
}
