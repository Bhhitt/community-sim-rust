use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MLPConfig {
    pub input_size: usize,
    pub hidden_sizes: Vec<usize>,
    pub output_size: usize,
    pub weights: Vec<Vec<Vec<f32>>>, // Each layer's weights: Vec<layer>[Vec<output_neuron>[Vec<input_neuron>]]
    pub biases: Vec<Vec<f32>>,  // Each layer's biases
}

#[derive(Clone, Debug)]
pub struct MLP {
    pub layers: Vec<(Vec<Vec<f32>>, Vec<f32>)>, // (weights, biases) for each layer
}

impl MLP {
    pub fn from_config(config: &MLPConfig) -> Self {
        let layers = config.weights.iter().zip(&config.biases)
            .map(|(w, b)| (w.clone(), b.clone()))
            .collect();
        MLP { layers }
    }

    pub fn forward(&self, mut input: Vec<f32>) -> Vec<f32> {
        for (layer_idx, (weights, biases)) in self.layers.iter().enumerate() {
            let mut output = vec![0.0; biases.len()];
            for (j, bias) in biases.iter().enumerate() {
                output[j] = *bias;
                for (i, inp) in input.iter().enumerate() {
                    output[j] += weights[j][i] * inp;
                }
                // Simple ReLU activation except for last layer
                if layer_idx != self.layers.len() - 1 {
                    output[j] = output[j].max(0.0);
                }
            }
            input = output;
        }
        input
    }
}
