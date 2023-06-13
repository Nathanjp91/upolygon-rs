use pyo3::prelude::*;
use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};


#[pyfunction]
fn rle_encode(data: Vec<Vec<u8>>) -> PyResult<Vec<u8>> {
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
    Ok(encoded)
}

/// A Python module implemented in Rust.
#[pymodule]
fn upolygon_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rle_encode, m)?)?;
    Ok(())
}