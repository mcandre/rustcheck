use std::fmt::Debug;

extern crate rand;
use rand::{Rng, Rand};
use rand::thread_rng as rng;



/// Wrapper around rand::Rand trait.
pub trait RustCheck<T> {
    /// Generates a random instance of type `T`.
    fn gen_rand() -> T;
}


// Implement RustCheck for relavent primitive types
macro_rules! primitive_impl {
    ($( $T:ty ),+) => {
        $(impl RustCheck<$T> for $T {
            fn gen_rand() -> $T {
                rng().gen()
            }
        })+
    }
}
primitive_impl!(bool,char,u8,u16,u32,u64,i8,i16,i32,i64,isize,usize,f32,f64);


impl<T> RustCheck<Vec<T>> for Vec<T> where T: Rand {
    /// Generate `Vec` of random length with random contents of type `T`.
    fn gen_rand() -> Vec<T> {
        rng().gen_iter::<T>().take(rng().gen::<u8>() as usize).collect()
    }
}


impl RustCheck<String> for String {
    /// Generate `String` of random length with random UTF-8 chars.
    fn gen_rand() -> String {
        rng().gen_iter::<char>().take(rng().gen::<u8>() as usize).collect()    
    } 
}


/// Simply a wrapper around `T::gen_rand()` to make this more similar to how it is done in other
/// QuickCheck implmentations.
#[inline]
pub fn gen<T>() -> T where T: RustCheck<T> {
    T::gen_rand()
}


/// Tests a property against a series of generator functions with `iters` random values inputted
/// into each.
///
/// # Arguments
///
/// * `prop`  - the property being tested
/// * `gens`  - a vector of functions to be tested against the property
/// * `iters` - the number of trials to run on each generator function
/// 
/// # Returns
///
/// A result with the error field set to the value that caused `prop` to return false.
///
///
/// # Examples
/// ```
/// // Property that conceptually *should* return true for all digits.
/// let p = |x| x < x + 1;
///
/// let res = rustcheck::for_all(p, vec![rustcheck::gen::<i8>], 1000);
/// // If this is a debug build, this will fail anyway because of integer overflow
/// if res.is_err() { assert_eq!(res.unwrap_err(), 128) }
///
/// ```
/// In this case, rustcheck detected that in a signed 8-bit integer, 128 + 1 = -127
/// (thus failing the even property).
pub fn for_all<T, F, G>(prop: F, gens: Vec<G>, iters: usize) -> Result<(), T>
where T: Clone + Debug,
      F: Fn(T) -> bool,
      G: Fn() -> T {
    for _ in 0..iters {
        for g in &gens {
            let v = g();
            if !prop(v.clone()) {
                return Err(v);
            }
        }
    }
    Ok(())
}

/// Macro that panics if for_all fails, printing out the value that caused it to fail
#[macro_export]
macro_rules! for_all {
    ($prop:expr, $gens:expr) => {
        $crate::for_all($prop, $gens).unwrap_or_else(|e| panic!(format!("{:?}", e)));
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::i8;
    fn gen_even() -> i8 {
        let i: i8 = gen();
        if i % 2 == 0 { i } else if i == i8::MAX {i - 1 } else { i + 1 }
    }
    #[test]
    fn check_evens() {
        let prop_even = |x| x % 2 == 0;
        for_all!(prop_even, vec![gen_even]);
    }
}

