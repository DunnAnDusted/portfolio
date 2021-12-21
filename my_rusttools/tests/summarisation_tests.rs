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
fn it_works() {
    let a = ["One", "Two", "Three", "Three"];
    let most_common = a.iter().most_common();
        assert!(most_common.is_some());
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