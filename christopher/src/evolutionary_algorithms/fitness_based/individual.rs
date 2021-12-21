use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Individual<T> 
where
    T: Eq, {
    genome: Vec<T>,
    fitness: u32,

}

impl<T> PartialOrd for Individual<T>
where
    T: Eq, {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.fitness.cmp(&other.fitness))
        }
    }

impl<T> Ord for Individual<T>
where
    T: Eq, {
        fn cmp(&self, other: &Self) -> Ordering {
            self.fitness.cmp(&other.fitness)
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord_test() {
        let test = vec![Individual{genome: vec!['y'], fitness: 0}, Individual{genome: vec!['n'], fitness: 10}];

        println!("{:#?}", test.iter().max());
    }
}