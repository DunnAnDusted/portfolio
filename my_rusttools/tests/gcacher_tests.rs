#![allow(unused_assignments, unused_variables)]
use std::collections::HashMap;

use my_rusttools::GCacher;
    
#[test]
fn it_works() {
    let mut cache = GCacher::new(|x: &usize|x + 1);

    assert_eq!(&2, cache.value_from(1));
    assert_eq!(&2, cache.value_from(1));
}

#[test]
fn get_value_test() {
    let mut cache = GCacher::new(|x: &usize|x * x);
    cache.value_from(2);

    let cache = cache;

    assert_eq!(Some(&4), cache.get(&2));
}

#[test]
fn deref_test() {
    let cache = GCacher::new(|x: &usize|x * x);
    let __test = &*cache;
}

#[test]
fn capture_test() {
    let mut num = 0;
    num += 1;
    let mut cache = GCacher::new(|x: &usize|x + num);

    cache.value_from(2);
    num += 1;
}

#[test]
fn type_anno_test() {
    let mut test = GCacher::new(|x: &&str|x.bytes());
    test.value_from(&"Test".to_string());
}

#[test]
fn deconstruction_test() {
    let mut num = 0;
    num += 1;
    let cache = GCacher::new(|x: &usize|x + num);

    let instancer = cache.instancer;
    // Invalid incriment. Causes an error.
    // num += 1;
    let cache = GCacher::new(instancer);
    drop(cache);
    num += 1;
    let mut cache = GCacher::new(|x: &String|x.to_lowercase());

    let instancer = cache.instancer;
    println!("{}", instancer(&"Test".to_string()));
    println!("{}", cache.value_from("Test".to_string()));
}

#[test]
fn into_test() {
    let cache = GCacher::new(|x: &usize|x * x);
    let test: HashMap<_, _> = cache.into();
    let cache = GCacher::new(|x: &usize|x * x);
    let (t1, t2) = cache.into();
}