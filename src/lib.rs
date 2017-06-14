//! `to_vec`, to_set and`to_map` are specializations of `collect` in the
//! usual case where you do want these containers.
//!
//! ```
//! use to_vec::ToVec;
//!
//! let v = "one two three".split_whitespace().to_vec();
//! assert_eq!(v,&["one","two","three"]);
//! ```
//! There's a specialized form for collecting `Result<T,E>` into
//! `Result<Vec<T>,E>`, where the error is the _first_ error encountered.
//!
//! ```
//! use to_vec::ToVecResult;
//!
//! let numbers = "23E 5F5 FF00".split_whitespace()
//!     .map(|s| u32::from_str_radix(s,16)).to_vec_result().unwrap();
//!
//! assert_eq!(numbers,&[0x23E, 0x5F5, 0xFF00]);
//! ```
//!
//! `to_map` and `to_set` are different - they operate on iterators
//! of _references_ and implicitly clone this.
//!
//! ```
//! use to_vec::ToMap;
//! const VALUES: &[(&str,i32)] = &[("hello",10),("dolly",20)];
//!
//! let map = VALUES.iter().to_map();
//!
//! assert_eq!(map.get("hello"),Some(&10));
//! assert_eq!(map.get("dolly"),Some(&20));
//! ```
//!
//! This implicit cloning behaviour is very useful for sets (here defined
//! as `HashSet`):
//!
//! ```
//! use to_vec::ToSet;
//!
//! let colours = ["green","orange","blue"].iter().to_set();
//! let fruit = ["apple","banana","orange"].iter().to_set();
//! let common = colours.intersection(&fruit).to_set();
//! assert_eq!(common, ["orange"].iter().to_set());
//! ```

use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use std::cmp::Eq;
use std::hash::Hash;
use std::result::Result;

/// to_vec() method on iterators
pub trait ToVec<T> {
    /// a more definite alternative to `collect`
    /// which collects an iterator's values into a Vec
    fn to_vec(self) -> Vec<T>;
}

/// to_vec_result() method on iterators
pub trait ToVecResult<T,E> {
    /// this collects an iterator of `Result<T,E>`
    /// into a result of `Result<Vec<T>,E>`
    fn to_vec_result(self) -> Result<Vec<T>,E>;
}

/// to_map() method on iterators of references
pub trait ToMap<K,V> {
    /// collect references into a HashMap by cloning
    fn to_map(self) -> HashMap<K,V>;
}

/// to_set() method on iterators of references
pub trait ToSet<K> {
    /// collect values into a HashSet by cloning
    fn to_set(self) -> HashSet<K>;
}

impl <T,I> ToVec<T> for I
where I: Iterator<Item=T> {
    fn to_vec(self) -> Vec<T> {
        FromIterator::from_iter(self)
    }
}

impl <T,E,I> ToVecResult<T,E> for I
where I: Iterator<Item=Result<T,E>> {
    fn to_vec_result(self) -> Result<Vec<T>,E> {
        FromIterator::from_iter(self)
    }
}

impl <'a, K,V,I> ToMap<K,V> for I
where K: Eq + Hash + Clone +'a, V: Clone +'a, I: Iterator<Item=&'a (K,V)>   {
    fn to_map(self) -> HashMap<K,V> {
        FromIterator::from_iter(self.cloned())
    }
}


impl <'a, K,I> ToSet<K> for I
where K: Eq + Hash + Clone + 'a, I: Iterator<Item=&'a K>   {
    fn to_set(self) -> HashSet<K> {
        FromIterator::from_iter(self.cloned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vec() {
        let v = "one two three".split_whitespace().to_vec();
        assert_eq!(v,&["one","two","three"]);
    }

    #[test]
    fn test_to_vec_result() {
        let numbers = "23E 5F5 FF00".split_whitespace()
            .map(|s| u32::from_str_radix(s,16)).to_vec_result().unwrap();

        assert_eq!(numbers,&[0x23E, 0x5F5, 0xFF00]);
    }

    #[test]
    fn test_to_set() {
        let set1 = [10,5,2,5,10].iter().to_set();
        let set2 = [2,5,10].iter().to_set();

        assert_eq!(set1,set2);

        let set3 = set1.intersection(&set2).to_set();
        assert_eq!(set3,set1);

        let colours = ["green","orange","blue"].iter().to_set();
        let fruit = ["apple","banana","orange"].iter().to_set();
        let common = colours.intersection(&fruit).to_set();
        assert_eq!(common, ["orange"].iter().to_set());
    }

    const VALUES: &[(&str,i32)] = &[("hello",10),("dolly",20)];

    #[test]
    fn test_to_map() {

        let map = VALUES.iter().to_map();

        assert_eq!(map.get("hello"),Some(&10));
        assert_eq!(map.get("dolly"),Some(&20));

    }


}
