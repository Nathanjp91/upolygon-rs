use pyo3::prelude::*;
use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};

#[derive(FromPyObject)]
enum Mask {
    SingleD(Vec<u64>),
    MultiD(Vec<Vec<u64>>),
} 

#[pyfunction]
fn rle_encode(data: Mask) -> PyResult<Vec<u64>> {
    match data {
        Mask::SingleD(data) => return Ok(rle_encode_1d(data)),
        Mask::MultiD(data) => return Ok(rle_encode_2d(data)),
    }
}

fn rle_encode_1d(data: Vec<u64>) -> Vec<u64> {
    let mut encoded = Vec::new();
    let mut count = 0;
    let mut prev = data[0];
    for col in data {
        if col == prev {
            count += 1;
        } else {
            encoded.push(count);
            encoded.push(prev);
            count = 1;
            prev = col;
        }
    }
    encoded.push(count);
    encoded.push(prev);
    encoded
}

fn rle_encode_2d(data: Vec<Vec<u64>>) -> Vec<u64> {
    let mut encoded = Vec::new();
    let mut count = 0;
    let mut prev = data[0][0];
    for row in data {
        for col in row {
            if col == prev {
                count += 1;
            } else {
                encoded.push(count);
                encoded.push(prev);
                count = 1;
                prev = col;
            }
        }
    }
    encoded.push(count);
    encoded.push(prev);
    encoded
}

/// A Python module implemented in Rust.
#[pymodule]
fn upolygon_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rle_encode, m)?)?;
    Ok(())
}