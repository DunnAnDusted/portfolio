#![allow(unused_assignments, unused_variables)]
use my_rusttools::traits::{
    SummariseCollection,
    TallyItems,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct TestType;

#[derive(Clone, PartialEq, Eq, Debug)]
struct TestType2;

#[test]
fn count_items_behaviour() {
    let a = [1, 2, 1, 2, 3, 4];
    let b: [i32; 0] = [];

    let count_a = a.iter().count_items();
    assert!(!count_a.is_empty());

    let count_b = b.iter().count_items();
    assert!(count_b.is_empty());
}

#[test]
fn most_common_behaviour() {
    let a = ["One", "Two", "Three", "Three"];
    let b: [&str; 0] = [];

    let most_common_a = a.iter().most_common_count();
    assert!(matches!(most_common_a, Some((_, 2))));

    let most_common_b = b.iter().most_common_count();
    assert!(most_common_b.is_none());
}

#[test]
fn least_common_behaviour() {
    let a = ["One", "Two", "Two", "Three", "Three"];
    let b: [&str; 0] = [];

    let least_common_a = a.iter().least_common_count();
    assert!(matches!(least_common_a, Some((_, 1))));

    let least_common_b = b.iter().least_common_count();
    assert!(least_common_b.is_none());
}

#[test]
fn tally_test() {
    let a = ["One", "Two", "Three", "Three"];
    let b = [TestType; 4];
    let c = [TestType2, TestType2];

    assert_eq!(2, a.iter().tally_item(&"Three"));
    assert_eq!(4, b.iter().tally_item(&TestType));
    assert_eq!(2, c.iter().tally_item(&TestType2));

    println!("{:?}", c[0]);
}