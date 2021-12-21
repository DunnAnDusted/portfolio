use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    bias: f64,
    biasw: f64,
    w: Vec<f64>,
    rate: f64,
    f: Activation,
    last_pulse: f64,
    last_input: Vec<f64>,
    last_output: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum Activation {
    Threshold,
    PWL,
    Sigamoid,
}

impl Neuron {
        pub fn new(count: usize, activation: Activation, learn: f64) -> Neuron {
            if count > 0 {
                let mut rng = rand::thread_rng();
                let mut temp = vec![0.0; count];
                temp.fill_with(||rng.gen());
                Self {
                    bias: rng.gen(),
                    biasw: rng.gen(),
                    w: temp,
                    rate: learn,
                    f: activation,
                    last_pulse: Default::default(),
                    last_input: Default::default(),
                    last_output: Default::default(),
                }
            } else {
                panic!("input count must be 1 or greater");
            }
        }

        pub fn pulse(&mut self, x: &Vec<f64>) -> f64 {
            x.iter().for_each(|x|if *x > 1.0 || *x < 0.0 {
                panic!("input outside valid range (0-1)");
            });

            if x.len() != self.w.len() {
                panic!("input lengths mismatch");
            }

            self.last_input = x.clone();

            self.last_pulse = self.last_input.iter()
                .zip(&self.w)
                .fold(0.0, |acc, (x, y)| acc + x*y) + self.bias * self.biasw;


            self.last_output = match self.f {
                Activation::Threshold => if self.last_pulse >= 0.0 { 1.0 } else { 0.0 },
                Activation::PWL => {
                    if self.last_pulse >= 0.5 { 
                        1.0 
                    } else if -0.5 <= self.last_pulse && self.last_pulse <= 0.5 { 
                        self.last_pulse + 0.5
                    } else {
                        0.0
                    }
                }
                Activation::Sigamoid => {
                    1.0/(1.0 + std::f64::consts::E.powf(-self.last_pulse))
                }
            };

            self.last_output
        }

        pub fn eval_last_pulse(&mut self, target: f64) -> bool {
            let matches = self.last_output != target;

            if matches {
                let error = target - self.last_output;
                self.w.iter_mut()
                    .zip(self.last_input.iter())
                    .for_each(|(w, x)|*w += error * x * self.rate);
                self.biasw += error * self.bias;
            }

            matches
        }

        pub fn train(&mut self, x: &Vec<f64>, target: f64) -> (f64, bool) {
            (self.pulse(x), self.eval_last_pulse(target))
        }

        pub fn bias(&self) -> f64 {
            self.bias
        }

        pub fn bias_weight(&self) -> f64 {
            self.biasw
        }

        pub fn wighted_bias(&self) -> f64 {
            self.bias * self.biasw
        }

        pub fn input_weights(&self) -> &[f64] {
            self.w.as_slice()
        }

        pub fn learning_rate(&self) -> f64 {
            self.rate
        }

        pub fn activation_type(&self) -> Activation {
            self.f
        }

        pub fn last_pulse(&self) -> f64 {
            self.last_pulse
        }

        pub fn last_input(&self) -> &[f64] {
            self.last_input.as_slice()
        }

        pub fn last_output(&self) -> f64 {
            self.last_output
        }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn summary() {
        let mut neuron = Neuron::new(2, Activation::Threshold, 0.05);
        println!("Pulse: {}", neuron.pulse(&vec![0.0, 1.0]));
        println!("{:#?}", neuron);
    }
}