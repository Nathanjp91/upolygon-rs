use pyo3::{prelude::*, AsPyPointer};
use numpy::ndarray::{ArrayD, Array1, Array2};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};

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

#[pyfunction]
fn rle_decode<'py>(py: Python<'py>, data: Vec<u64>, width: Option<usize>, height: Option<usize>) -> &'py PyArrayDyn<u64> {
    if width.unwrap_or(1) == 1 {
        let data = rle_decode_1d(data);
        let nddata = Array1::from_vec(data).into_dyn();
        return nddata.into_pyarray(py);
    } else {
        let data = rle_decode_2d(data, width.unwrap_or(1), height.unwrap_or(0));
        let mut nddata = Array2::<u64>::default((data.len(), data[0].len()));
        for (i, row) in data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                nddata[[i, j]] = *col;
            }
        }
        return nddata.into_dyn().into_pyarray(py);
    }
}

fn rle_decode_1d(data: Vec<u64>) -> Vec<u64> {
    let mut size = 0;
    for chunk in data.chunks_exact(2) {
        size += chunk[0];
    }
    let mut decoded = vec![0; size as usize];
    let mut index = 0;
    println!("{:?}", size);
    println!("{:?}", decoded.len());
    for chunk in data.chunks_exact(2) {
        let count = chunk[0];
        let value = chunk[1];
        for _ in 0..count {
            decoded[index] = value;
            index += 1;
        }
    }
    decoded
}

fn rle_decode_2d(data: Vec<u64>, width: usize, height: usize) -> Vec<Vec<u64>> {
    let mut decoded = vec![vec![0; height as usize]; width as usize];
    let mut row_count = 0;
    let mut col_count = 0;
    for chunk in data.chunks_exact(2) {
        let count = chunk[0];
        let value = chunk[1];
        for _ in 0..count {
            if row_count == width {
                decoded[row_count][col_count] = value;
                row_count += 1;
                col_count = 0;
            }
        }
    }
    decoded
}

/// A Python module implemented in Rust.
#[pymodule]
fn upolygon_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rle_encode, m)?)?;
    m.add_function(wrap_pyfunction!(rle_decode, m)?)?;
    Ok(())
}