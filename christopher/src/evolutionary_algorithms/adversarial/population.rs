use std::marker::PhantomData;

pub trait Population<T, U>
where
    U: Population<T, U>, {
        fn interact(&mut self, env: &mut T, others: &mut [&mut U]);
    }

pub enum Forresters<T, U, V, F, G> 
where
    T: PartialEq,
    U: Population<V, U> + PartialEq,
    F: FnMut(&mut [Option<HerdMember<T>>], &mut V, &mut [&mut U]),
    G: FnMut(&mut [Option<PackMember<T>>], &mut V, &mut [&mut U]), {
        Prey(Herd<T, U, V, F>),
        Predators(Pack<T, U, V, G>),
    }

impl<T, U, V, F, G> Population<V, U> for Forresters<T, U, V, F, G> 
where
    T: PartialEq,
    U: Population<V, U> + PartialEq,
    F: FnMut(&mut [Option<HerdMember<T>>], &mut V, &mut [&mut U]),
    G: FnMut(&mut [Option<PackMember<T>>], &mut V, &mut [&mut U]), {
        fn interact(&mut self, env: &mut V, others: &mut [&mut U]) {
            match self {
                Forresters::Prey(herd) => (herd.interaction)(herd.population.as_mut_slice(), env, others),
                Forresters::Predators(pack) => (pack.interaction)(pack.population.as_mut_slice(), env, others)
            }
        }
    }

#[derive(Debug, PartialEq)]
pub struct Herd<T, U, V, F> 
where
    T: PartialEq,
    U: Population<V, U> + PartialEq,
    F: FnMut(&mut [Option<HerdMember<T>>], &mut V, &mut [&mut U]) {
        population: Vec<Option<HerdMember<T>>>,
        interaction: F,
        env: PhantomData<V>,
        others: PhantomData<U>,
    }

#[derive(Debug, PartialEq)]
pub struct HerdMember<T: PartialEq> {
    genome: Vec<T>,
    fullness: u8,
}

#[derive(Debug, PartialEq)]
pub struct Pack<T, U, V, F> 
where
    T: PartialEq,
    U: Population<V, U> + PartialEq,
    F: FnMut(&mut [Option<PackMember<T>>], &mut V, &mut [&mut U]) {
        population: Vec<Option<PackMember<T>>>,
        interaction: F,
        env: PhantomData<V>,
        others: PhantomData<U>,
    }

#[derive(Debug, PartialEq)]
pub struct PackMember<T: PartialEq> {
    genome: Vec<T>,
    full: bool,
}