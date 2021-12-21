use christopher::neural_networks::*;

#[test]
fn and_test() {
    let mut neuron = Neuron::new(2, Activation::Threshold, 0.005);
    let truth_1: Vec<f64> = vec![0.0, 0.0];
    let truth_2: Vec<f64> = vec![1.0, 0.0];
    let truth_3: Vec<f64> = vec![0.0, 1.0];
    let truth_4: Vec<f64> = vec![1.0, 1.0];

    for _ in 0..=1000 {
        neuron.train(&truth_1, 0.0);
        neuron.train(&truth_2, 0.0);
        neuron.train(&truth_3, 0.0);
        neuron.train(&truth_4, 1.0);
    }

    println!("AND Training Test:\n\n[0, 0]: {}\n[1, 0]: {}\n[0, 1]: {}\n[1, 1]: {}",
        neuron.pulse(&truth_1),
        neuron.pulse(&truth_2),
        neuron.pulse(&truth_3),
        neuron.pulse(&truth_4)
    );
}

#[test]
fn or_test() {
    let mut neuron = Neuron::new(2, Activation::Threshold, 0.005);
    let truth_1: Vec<f64> = vec![0.0, 0.0];
    let truth_2: Vec<f64> = vec![1.0, 0.0];
    let truth_3: Vec<f64> = vec![0.0, 1.0];
    let truth_4: Vec<f64> = vec![1.0, 1.0];

    for _ in 0..=1000 {
        neuron.train(&truth_1, 0.0);
        neuron.train(&truth_2, 1.0);
        neuron.train(&truth_3, 1.0);
        neuron.train(&truth_4, 1.0);
    }

    println!("OR Training Test:\n\n[0, 0]: {}\n[1, 0]: {}\n[0, 1]: {}\n[1, 1]: {}",
        neuron.pulse(&truth_1),
        neuron.pulse(&truth_2),
        neuron.pulse(&truth_3),
        neuron.pulse(&truth_4)
    );
}

#[test]
fn not_test() {
    let mut neuron = Neuron::new(1, Activation::Threshold, 0.005);
    let not_1: Vec<f64> = vec![0.0];
    let not_2: Vec<f64> = vec![1.0];

    for _ in 0..=1000 {
        neuron.train(&not_1, 1.0);
        neuron.train(&not_2, 0.0);
    }

    println!("NOT Training Test:\n\n[0]: {}\n[1]: {}",
        neuron.pulse(&not_1),
        neuron.pulse(&not_2),
    );
}

#[test]
fn sig_test() {
    let mut neuron = Neuron::new(2, Activation::Sigamoid, 0.005);
    let truth_1: Vec<f64> = vec![0.0, 0.0];
    let truth_2: Vec<f64> = vec![1.0, 0.0];
    let truth_3: Vec<f64> = vec![0.0, 1.0];
    let truth_4: Vec<f64> = vec![1.0, 1.0];

    for _ in 0..=1000 {
        neuron.train(&truth_1, 0.0);
        neuron.train(&truth_2, 0.0);
        neuron.train(&truth_3, 0.0);
        neuron.train(&truth_4, 1.0);
    }

    println!("SIGAMOID Test:\n\n[0, 0]: {}\n[1, 0]: {}\n[0, 1]: {}\n[1, 1]: {}",
        neuron.pulse(&truth_1),
        neuron.pulse(&truth_2),
        neuron.pulse(&truth_3),
        neuron.pulse(&truth_4),
    )
}

/*fn train_gate(neuron: &mut Neuron, expected: &[f64], iterations: usize) {
    let inputs: Vec<Vec<bool>> = get_truth_table(neuron.input_weights().len() as u32);

    for _ in 0..=iterations {
        inputs.iter()
            .map(|x|{
                x.iter()
                    .map(|y|*y as u8 as f64)
                    .collect()
            })
            .zip(expected.iter())
            .for_each(|(x, y)|{
                neuron.train(&x, *y);
            })
    }
}

fn get_truth_table(inputs: u32) -> Vec<Vec<bool>> {
    let row_count = (2 as usize).pow(inputs);
    let mut table: Vec<Vec<bool>> = Vec::with_capacity(row_count);

    table.push(vec![false; inputs as usize]);

    for i in 1..row_count - 1 {
        let mut temp: Vec<f64> = Vec::new();


    }

    table.push(vec![true; inputs as usize]);

    table
}*/