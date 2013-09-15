use rustcheck;
import rustcheck::*;

use std;

fn prop_even(x : int) -> bool {
  ret x % 2 == 0;
}

fn gen_even() -> int {
  let i : int = rustcheck::gen_int();

  if i % 2 == 0 {
    ret i;
  }
  else {
    ret i + 1;
  }
}

fn main() {
  for_all(prop_even, [gen_int]);
  for_all(prop_even, [gen_even]);
}
