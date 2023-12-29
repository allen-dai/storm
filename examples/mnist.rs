use num_traits::FromPrimitive;
use kdam::{tqdm, BarExt};
use rand::{seq::SliceRandom, thread_rng};
use storm::{prelude::*, nn::optim::{adam, Optimizer}};

pub fn main() {
    let training = true;
    let mut model = ConvNet::default();
    if training {
        let optim = adam(&[&mut model.c1, &mut model.c2, &mut model.l1], 0.001);
        let batch_size = 128;
        train(&model, optim, batch_size, 60000/batch_size).unwrap();
    } else {
        eval(&model).unwrap();
    }
}

pub struct ConvNet {
    pub c1: Tensor,
    pub c2: Tensor,
    pub l1: Tensor,
    pub l2: Tensor,
}

impl Default for ConvNet {
    fn default() -> Self {
        let conv = 3;
        Self {
            c1: Tensor::scaled_uniform([32, 1, conv, conv]),
            c2: Tensor::scaled_uniform([64, 32, conv, conv]),
            l1: Tensor::scaled_uniform([9216, 128]),
            l2: Tensor::scaled_uniform([128, 1]),
        }
    }
}

impl ConvNet {
    fn forward(&self, x: &Tensor) -> Tensor {
        let d1 = x.shape().numel() / 28 / 28;
        let mut y = x.reshape([d1, 1, 28, 28]);
        y = y.conv2d(&self.c1).sigmoid();
        y = y.conv2d(&self.c2).sigmoid().flatten().reshape([-1, self.l1.shape()[0]]);
        y = y.matmul(&self.l1).sigmoid();
        y = y.matmul(&self.l2).sigmoid();
        y
    }

    // fn save(&self, path: &str) -> Result<(), safetensors::SafeTensorError> {
    //     Tensor::to_safetensor(
    //         &[("c1", &self.c1), ("c2", &self.c2), ("l1", &self.l1)],
    //         path,
    //     )?;
    //     Ok(())
    // }
    //
    // fn load(&mut self, path: &str) -> Result<(), safetensors::SafeTensorError> {
    //     self.c1.from_file("c1", path)?;
    //     self.c2.from_file("c2", path)?;
    //     self.l1.from_file("l1", path)?;
    //     Ok(())
    // }
}
fn fetch_mnist_shuffled(
    batch_size: usize,
) -> (
    Vec<Vec<f32>>,
    Vec<Vec<f32>>,
    Vec<Vec<f32>>,
    Vec<Vec<f32>>,
) {
    use mnist::Mnist;
    let mnist = Mnist::from_download().expect("mnist download failed");
    let Mnist {
        train_images,
        train_labels,
        test_images,
        test_labels,
    } = mnist;
    let mut rng = thread_rng();

    // batching train
    let mut shuffle_idx: Vec<usize> = (0..60000).collect();
    shuffle_idx.shuffle(&mut rng);
    let mut train_img_batched: Vec<Vec<f32>> = Vec::with_capacity(60000 * 28 * 28);
    let mut train_lbl_batched: Vec<Vec<f32>> = Vec::with_capacity(60000 * 10);
    let mut tain_img_in_one_batch = Vec::with_capacity(batch_size);
    let mut train_lbl_in_one_batch = Vec::with_capacity(batch_size);
    for i in 0..60000 {
        for ii in 0..28 * 28 {
            tain_img_in_one_batch
                .push(f32::from_u8(train_images[(shuffle_idx[i] * (28 * 28)) + ii]).unwrap());
        }
        train_lbl_in_one_batch.push(f32::from_u8(train_labels[shuffle_idx[i]]).unwrap());
        if (i + 1) % batch_size == 0 {
            train_img_batched.push(tain_img_in_one_batch.drain(..).collect::<Vec<f32>>());
            train_lbl_batched.push(train_lbl_in_one_batch.drain(..).collect::<Vec<f32>>());
        }
    }

    // batching test
    let mut shuffle_idx: Vec<usize> = (0..10000).collect();
    shuffle_idx.shuffle(&mut rng);
    let mut test_img_batched: Vec<Vec<f32>> = Vec::with_capacity(10000 * 28 * 28);
    let mut test_lbl_batched: Vec<Vec<f32>> = Vec::with_capacity(10000 * 10);
    let mut test_img_in_one_batch = Vec::with_capacity(batch_size);
    let mut test_lbl_in_one_batch = Vec::with_capacity(batch_size);
    for i in 0..10000 {
        for ii in 0..28 * 28 {
            test_img_in_one_batch
                .push(f32::from_u8(test_images[(shuffle_idx[i] * (28 * 28)) + ii]).unwrap());
        }
        test_lbl_in_one_batch.push(f32::from_u8(test_labels[shuffle_idx[i]]).unwrap());
        if (i + 1) % batch_size == 0 {
            test_img_batched.push(test_img_in_one_batch.drain(..).collect::<Vec<f32>>());
            test_lbl_batched.push(test_lbl_in_one_batch.drain(..).collect::<Vec<f32>>());
        }
    }
    (
        train_img_batched,
        train_lbl_batched,
        test_img_batched,
        test_lbl_batched,
    )
}

fn train<Optim: Optimizer>(
    model: &ConvNet,
    mut optim: Optim,
    batch_size: usize,
    epoch: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let (img_batched, lbl_batched, _, _) = fetch_mnist_shuffled(batch_size);
    let mut pb = tqdm!(total = epoch);
    pb.set_description(format!("loss: {:.2} accuracy {:.2}", 0, 0));
    pb.refresh()?;
    for i in 0..epoch {
        let x = Tensor::from(&*img_batched[i]).reshape([batch_size, 1, 28, 28]);
        let y = Tensor::from(&*lbl_batched[i]).reshape([batch_size]);
        let mut out = model.forward(&x);
        //out.backward();
        let mut loss = (out - y).abs().mean();
        loss.realize();
        println!("loss {:?}",loss.to_vec());
        //loss.backward();
        //loss.realize();
        // optim.step();
        // let pred = out.detach().argmax(-1);
        // let accuracy = (pred._eq(&y.detach())).mean();
        // pb.set_description(format!(
        //     "loss: {:.2?} accuracy {:.2?}",
        //     loss.to_vec()[0],
        //     accuracy.to_vec()[0]
        // ));
        pb.update(1)?;
    }
    Ok(())
}

fn eval(model: &ConvNet) -> Result<(), Box<dyn std::error::Error>> {
    let batch_size = 128;
    let (_, _, img_batched, lbl_batched) = fetch_mnist_shuffled(batch_size);
    let mut pb = tqdm!(total = 50);
    pb.set_description(format!("eval accuracy {:.2}", 0));
    pb.refresh()?;
    for i in 0..50 {
        let x = Tensor::from(&*img_batched[i]).reshape([batch_size, 1, 28, 28]);
        let y = Tensor::from(&*lbl_batched[i]).reshape([batch_size]);
        let out = model.forward(&x);
        let pred = out.detach().argmax(-1);
        let accuracy = (pred._eq(&y.detach())).mean();
        pb.set_description(format!("eval accuracy {:.2?}", accuracy.to_vec()[0]));
        pb.update(1)?;
    }
    Ok(())
}