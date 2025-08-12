use ndarray::{s, Array, ArrayView1, Array2, Axis};
use ndarray_rand::rand_distr::{Uniform, Normal};
use ndarray_rand::RandomExt;
use std::{error::Error, fs::File};

fn sigmoid(X: Array2<f64>) -> Array2<f64> {
    X.clone().mapv(|x| 1.0 / (1.0 + (-x).exp()))
}

fn sigmoid_derivative(X: Array2<f64>) -> Array2<f64> {
    let d = X.clone() * &(1.0 - X.clone());
    // println!("Sigmoid derivative sample: {:?}", d.slice(s![0, ..]));
    return d;
}

fn softmax(array: Array2<f64>) -> Array2<f64> {
    let mut result = array.clone();
    
    for mut row in result.axis_iter_mut(Axis(0)) {
        let exp_row: Vec<f64> = row.iter().map(|&x| x.exp()).collect();
        let sum: f64 = exp_row.iter().sum();
        for (i, value) in row.iter_mut().enumerate() {
            *value = exp_row[i] / sum;
        }
    }
    result
}

fn read_csv(path: &str) -> Result<(Array2<f64>, Array2<f64>), Box<dyn Error>> {
    let mut row_count = 0;
    let mut data_vec = Vec::new();
    let mut label_vec = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    let mut i = 0;
    for result in rdr.records() {
        let row = result?;
        for (j, elem) in row.iter().enumerate() {
            if j == 0 {
                let label: usize = elem.parse()?;
                for k in 0..10 {
                    label_vec.push(if k == label { 1.0 } else { 0.0 });
                }
            } else {
                let pixel: f64 = elem.parse()?;
                data_vec.push((pixel / 255.0));
            }
        }
        row_count += 1;
    }
    let data = Array2::from_shape_vec((row_count, 784), data_vec)?;
    let labels = Array2::from_shape_vec((row_count, 10), label_vec)?;
    Ok((data, labels))
}

fn argmax(arr: &ArrayView1<f64>) -> usize {
    arr.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i).unwrap()
}

struct NeuralNetwork {
    input_size: usize,
    layer1_size: usize,
    layer2_size: usize,
    output_size: usize,
    learning_rate: f64,
    weights_input_to_layer1: Array2<f64>,
    weights_layer1_to_layer2: Array2<f64>,
    weights_layer2_to_output: Array2<f64>,
}

impl NeuralNetwork {
    fn new(input_size: usize, layer1_size: usize, layer2_size: usize, output_size: usize, learning_rate: f64) -> Self {
        let weights_input_to_layer1 = Array::random((input_size, layer1_size), Uniform::new(-0.5, 0.5));
        let weights_layer1_to_layer2 = Array::random((layer1_size, layer2_size), Uniform::new(-0.5, 0.5));
        let weights_layer2_to_output = Array::random((layer2_size, output_size), Uniform::new(-0.5, 0.5));

        NeuralNetwork {
            input_size,
            layer1_size,
            layer2_size,
            output_size,
            learning_rate,
            weights_input_to_layer1,
            weights_layer1_to_layer2,
            weights_layer2_to_output,
        }
    }

    fn forward(&self, input: &Array2<f64>) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
        let l1_in = input.dot(&self.weights_input_to_layer1);
        // println!("l1_in sample: {:?}", l1_in.slice(s![0, 0..5]));
        let l1_out = sigmoid(l1_in);
        // println!("l1_out sample: {:?}", l1_out.slice(s![0, 0..5]));
        let l2_in = l1_out.dot(&self.weights_layer1_to_layer2);
        // println!("l2_in sample: {:?}", l2_in.slice(s![0, 0..5]));
        let l2_out = sigmoid(l2_in);
        let out_in = l2_out.dot(&self.weights_layer2_to_output);
        let final_out = softmax(out_in);
        return (l1_out, l2_out, final_out);
    }

    fn backward(&mut self, input: &Array2<f64>, l1_out: &Array2<f64>, l2_out: &Array2<f64>, final_out: &Array2<f64>, target: &Array2<f64>) {
        let delta_3 = target - final_out; // +=
        let grad_3 = l2_out.t().dot(&delta_3);
        let w3_before = self.weights_layer2_to_output.mean().unwrap();
        self.weights_layer2_to_output += &(&grad_3 * self.learning_rate);
        let w3_after = self.weights_layer2_to_output.mean().unwrap();

        let delta_2 = delta_3.dot(&self.weights_layer2_to_output.t()) * (sigmoid_derivative(l2_out.clone()));
        let grad_2 = l1_out.t().dot(&delta_2);
        self.weights_layer1_to_layer2 += &(&grad_2 * self.learning_rate);

        let delta_1 = delta_2.dot(&self.weights_layer1_to_layer2.t()) * (sigmoid_derivative(l1_out.clone()));
        let grad_1 = input.t().dot(&delta_1);
        self.weights_input_to_layer1 += &(&grad_1 * self.learning_rate);

        // w3_after - w3_before
    }

    fn train(&mut self, n:usize, input: &Array2<f64>, labels: &Array2<f64>) {
        let batch_size = 100;
        let n_samples = input.nrows();
        for epoch in (0..n) {
            let mut total_loss = 0.0;
            for i in (0..n_samples).step_by(batch_size) {
                let end = (i + batch_size).min(n_samples);
                let input_batch = input.slice(s![i..end, ..]).to_owned();
                let label_batch = labels.slice(s![i..end, ..]).to_owned();
                let (l1, l2, out) = self.forward(&input_batch);
                let loss = cross_entropy_loss(&out, &label_batch);
                self.backward(&input_batch, &l1, &l2, &out, &label_batch);
                total_loss += loss;
            }
            
            println!("Epoch {}\n loss: {:.4}", epoch + 1, total_loss / (n_samples as f64));
            println!(" train acc: {}", self.calculate_accuracy(input, labels));
        }
    }

    fn calculate_accuracy(&self, data: &Array2<f64>, labels: &Array2<f64>) -> f64 {
        let mut results = 0;
        let (_, _, final_output) = self.forward(data);
        for (pred_row, label_row) in final_output.rows().into_iter().zip(labels.rows()) {
            let pred_idx = argmax(&pred_row);
            let true_idx = argmax(&label_row);
            if pred_idx == true_idx {
                results += 1;
            }
        }
        let acc = results as f64 / data.nrows() as f64;
        // println!("Haha look at your accuracy {}", acc);
        return acc;
    }
}

fn cross_entropy_loss(predictions: &Array2<f64>, targets: &Array2<f64>) -> f64 {
    let epsilon = 1e-7;
    let mut total_loss = 0.0;
    let n_samples = predictions.nrows();

    for (pred_row, target_row) in predictions.outer_iter().zip(targets.outer_iter()) {
        for (pred, target) in pred_row.iter().zip(target_row.iter()) {
            total_loss -= target * (pred + epsilon).ln();
        }
    }

    total_loss / n_samples as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Current dir: {}", std::env::current_dir()?.display()); 
    let mut my_nn = NeuralNetwork::new(784, 128, 32, 10, 0.01);
    let (mut train_data, train_labels) = read_csv("../../mnist_train.csv")?;
    let (mut test_data, test_labels) = read_csv("../../mnist_test.csv")?;
    println!("train data slice: {}", train_data.slice(s![0, 300..400]));

    my_nn.train(3, &train_data, &train_labels); 
    let acc = my_nn.calculate_accuracy(&test_data, &test_labels);
    println!("test acc: {}", acc);
   

    Ok(())      
}