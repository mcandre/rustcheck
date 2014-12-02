#![crate_id(name = "rustcheck", vers = "0.0.1")]

extern crate std;

use std::rand::random;

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
	return std::rand::random::<uint>() % 256;
}

pub fn gen_char() -> char {
	return (std::rand::random::<u8>() % 128) as char;
}

pub fn gen_vec<T: Clone>(gen : (|| -> T), len : uint) -> Vec<T> {
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

pub fn gen_str() -> String {
	let len : uint = (gen_int() % 100) as uint;

  let chars : Vec<char> = gen_vec(gen_char, len);

  let u8vec : Vec<u8> = Vec::from_fn(chars.len(), |index| *(chars.get(index)) as u8);

  let u8s : &[u8] = u8vec.as_slice();

  let option_str : Option<&str> = std::str::from_utf8(u8s);

  let s : &str = option_str.unwrap();

  let string : String = s.to_string();

  return string;
}

pub fn for_all<T>(property : |T| -> bool, mut gens : Vec<|| -> T>) -> bool {
    let mut result = true;

    for g in gens.mut_iter() {
      let v : T = (*g)();
      result = result & property(v);
    }

    return result;
}
