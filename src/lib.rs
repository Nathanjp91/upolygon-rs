use pyo3::{prelude::*, AsPyPointer};
use numpy::ndarray::{ArrayD, Array1, Array2};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};



#[pyfunction]
fn rle_encode(data: PyReadonlyArrayDyn<u64>) -> PyResult<Vec<u64>> {
    let reshape = data.as_array().shape().to_vec().into_iter().product();
    let data = data.as_array().into_shape([reshape]).unwrap().to_owned();
    return Ok(rle_encode_1d(data));
}

fn rle_encode_1d(data: Array1<u64>) -> Vec<u64> {
    let mut encoded = vec![0; data.len()*2];
    let mut count = 0;
    let mut prev = data[0];
    let mut index = 0;
    for col in data {
        if col == prev {
            count += 1;
        } else {
            encoded[index] = count;
            encoded[index + 1] = prev;
            index += 2;
            count = 1;
            prev = col;
        }
    }
    encoded[index] = count;
    encoded[index + 1] = prev;
    encoded[..index + 2].to_vec()
}

#[pyfunction]
fn rle_decode<'py>(py: Python<'py>, data: Vec<u64>, width: Option<usize>, height: Option<usize>) -> &'py PyArrayDyn<u64> {
    if width.unwrap_or(1) == 1 {
        let data = rle_decode_1d(data);
        return data.into_dyn().into_pyarray(py);
    } else {
        let data = rle_decode_2d(data, width.unwrap_or(1), height.unwrap_or(0));
        return data.into_dyn().into_pyarray(py);
    }
}

fn rle_decode_1d(data: Vec<u64>) -> Array1<u64> {
    let mut size = 0;
    for chunk in data.chunks_exact(2) {
        size += chunk[0];
    }
    let mut decoded = Array1::<u64>::default(size as usize);
    let mut index = 0;
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

fn rle_decode_2d(data: Vec<u64>, width: usize, height: usize) -> Array2<u64> {
    let mut decoded = Array2::<u64>::default((width, height));
    // let mut decoded = vec![vec![0; height as usize]; width as usize];
    let mut row_count = 0;
    let mut col_count = 0;
    for chunk in data.chunks_exact(2) {
        let count = chunk[0];
        let value = chunk[1];
        for _ in 0..count {
            if row_count == width {
                decoded[[row_count, col_count]] = value;
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

#[cfg(test)]
mod tests {
    use ndarray::{Array1};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn rle_encode_1d_test() {
        let data = Array1::<u64>::from(vec![0, 1, 1, 1, 0, 0, 1, 0, 0, 0]);
        let result = super::rle_encode_1d(data);
        println!("{:?}", result);
        assert_eq!(result, vec![1, 0, 3, 1, 2, 0, 1, 1, 3, 0]);
    }
}