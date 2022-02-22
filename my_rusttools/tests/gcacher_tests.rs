use my_rusttools::GCacher;
    
#[test]
fn it_works() {
    let mut cache = GCacher::new(|x: &usize|x + 1);

    assert_eq!(2, *cache.value_from(1));
}

#[test]
fn immutable_lock() {
    let mut cache = GCacher::new(|x: &usize|x * x);
    cache.value_from(2);

    let cache = cache;

    assert_eq!(Some(&4), cache.get(&2));
}

#[test]
fn instancer_capturing() {
    let mut num = 0;
    num += 1;
    let mut cache = GCacher::new(|x: &usize|x + num);

    cache.value_from(2);
    // Invalid incriment. Causes an error.
    // num += 1;
    cache.value_from(3);
    num += 1;

    assert_eq!(num, 2);
}

#[test]
fn gcacher_instancer_deconstruction() {
    let cache = GCacher::new(|x: &usize|x * x);
    let instancer = cache.into_instancer();

    let mut cache = GCacher::new(instancer);

    let instancer = cache.instancer();
    assert_eq!(4, instancer(&2));
    assert_eq!(16, *cache.value_from(4));
    // `cache` mutation invalidates future calls of `instancer`.
    // println!("{}", instancer(&String::from("Test")));
}

#[test]
fn gcacher_cache_deconstruction() {
    let mut cache = GCacher::new(|x: &usize|x * x);
    cache.value_from(2);

    let cache = cache.into_cache();
    assert_eq!(cache.get(&2), Some(&4));
}

#[test]
fn gcacher_inner_deconstruction() {
    let mut cache = GCacher::new(|x:& usize|x * x);
    cache.value_from(2);

    let (instancer, cache) = cache.into_inner();
    assert_eq!(cache.get(&2), Some(&4));
    assert_eq!(instancer(&2), 4);
}