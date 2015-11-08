use std::fmt::Debug;

extern crate rand;
use rand::{Rng, Rand};
use rand::thread_rng as rng;


// Wrapper around rand::Rand trait
pub trait RustCheck<T> {
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


// Generate Vec of random length with random contents of type T
impl<T> RustCheck<Vec<T>> for Vec<T> where T: Rand {
    fn gen_rand() -> Vec<T> {
        rng().gen_iter::<T>().take(rng().gen::<u8>() as usize).collect()
    }
}


// Generate String of random length with random UTF-8 chars
impl RustCheck<String> for String {
    fn gen_rand() -> String {
        rng().gen_iter::<char>().take(rng().gen::<u8>() as usize).collect()    
    } 
}


#[inline]
pub fn gen<T>() -> T where T: RustCheck<T> {
    T::gen_rand()
}


pub fn for_all<T, F, G>(prop: F, gens: Vec<G>) -> Result<(), T>
where T: Clone + Debug,
      F: Fn(T) -> bool,
      G: Fn() -> T {

    for _ in 0..100 {
        for g in &gens {
            let v = g();
            if !prop(v.clone()) {
                return Err(v);
            }
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! for_all {
    ($prop:expr, $gens:expr) => {
        $crate::for_all($prop, $gens).unwrap_or_else(|e| panic!(format!("{:?}", e)));
    }
}


#[cfg(test)]
mod test {
    use super::*;
    fn gen_even() -> i32 {
        let i: i32 = gen();
        if i % 2 == 0 {i} else {i + 1}
    }
    #[test]
    fn check_evens() {
        let prop_even = |x| x % 2 == 0;
        for_all!(prop_even, vec![gen_even]);
    }
}

