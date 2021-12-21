use super::population::*;

pub struct Environment<T, U> 
where
    U: Population<T, U>, {
        environment_data: T,
        populations: Vec<U>,
    }

impl<T, U> Environment<T, U> 
where
    U: Population<T, U>, {
    pub fn new(env: T, pops: Vec<U>) -> Self {
        Self {
            environment_data: env,
            populations: pops,
        }
    }
}