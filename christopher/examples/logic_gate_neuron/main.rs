use christopher::neural_networks::*;

fn main() {
    let mut and = Neuron::new(2, Activation::Threshold, 0.001);
    let mut or = Neuron::new(2, Activation::Threshold, 0.001);
    let mut not = Neuron::new(1, Activation::Threshold, 0.001);
    let not_1: Vec<f64> = vec![0.0];
    let not_2: Vec<f64> = vec![1.0];
    let truth_1: Vec<f64> = vec![0.0, 0.0];
    let truth_2: Vec<f64> = vec![1.0, 0.0];
    let truth_3: Vec<f64> = vec![0.0, 1.0];
    let truth_4: Vec<f64> = vec![1.0, 1.0];

    for _ in 0..=1000 {
        and.train(&truth_1, 0.0);
        and.train(&truth_2, 0.0);
        and.train(&truth_3, 0.0);
        and.train(&truth_4, 1.0);
    }

    for _ in 0..=1000 {
        or.train(&truth_1, 0.0);
        or.train(&truth_2, 1.0);
        or.train(&truth_3, 1.0);
        or.train(&truth_4, 1.0);
    }

    for _ in 0..=1000 {
        not.train(&not_1, 1.0);
        not.train(&not_2, 0.0);
    }

    println!("AND Training Test:\n\n[0, 0]: {}\n[1, 0]: {}\n[0, 1]: {}\n[1, 1]: {}",
        and.pulse(&truth_1),
        and.pulse(&truth_2),
        and.pulse(&truth_3),
        and.pulse(&truth_4),
    );
    
    println!("OR Training Test:\n\n[0, 0]: {}\n[1, 0]: {}\n[0, 1]: {}\n[1, 1]: {}",
        or.pulse(&truth_1),
        or.pulse(&truth_2),
        or.pulse(&truth_3),
        or.pulse(&truth_4),
    );

    println!("NOT Training Test:\n\n[0]: {}\n[1]: {}",
        not.pulse(&not_1),
        not.pulse(&not_2),
    );
}
