use pyo3::{prelude::*};
use numpy::ndarray::{Array1, Array2};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn, PyArray1, PyArray2, PyReadonlyArray2};



#[pyfunction]
fn rle_encode<'py>(py: Python<'py>, data: PyReadonlyArrayDyn<u64>) -> &'py PyArray1<u64> {
    let reshape = data.as_array().shape().to_vec().into_iter().product();
    let data = data.as_array().into_shape([reshape]).unwrap().to_owned();
    return rle_encode_1d(data).into_pyarray(py);
}

#[derive(FromPyObject, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

struct Polygon {
    points: Vec<Point>,
    extents: (i64, i64, i64, i64),
    valid: bool,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        let extents = get_extents(points.clone());
        let valid = points[0].x == points[points.len() - 1].x && points[0].y == points[points.len() - 1].y;
        Self {
            points,
            extents,
            valid
        }
    }
}

fn rle_encode_1d(data: Array1<u64>) -> Array1<u64> {
    let mut encoded = vec![0; data.len() * 2];
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
    Array1::<u64>::from(encoded[..index+2].to_vec())
}

#[pyfunction]
fn rle_decode<'py>(py: Python<'py>, data: PyReadonlyArrayDyn<u64>, width: Option<usize>, height: Option<usize>) -> &'py PyArrayDyn<u64> {
    let data = data.as_array().into_shape([data.len()]).unwrap().to_owned();
    if width.unwrap_or(1) == 1 {
        let data = rle_decode_1d(data);
        return data.into_dyn().into_pyarray(py);
    } else {
        let data = rle_decode_2d(data, width.unwrap_or(1), height.unwrap_or(0));
        return data.into_dyn().into_pyarray(py);
    }
}

fn rle_decode_1d(data: Array1<u64>) -> Array1<u64> {
    let mut size = 0;
    for i in (0..data.len()).step_by(2) {
        size += data[i];
    }
    let mut decoded = Array1::<u64>::default(size as usize);
    let mut index = 0;
    for i in (0..data.len()).step_by(2) {
        let count = data[i];
        let value = data[i+1];
        for _ in 0..count {
            decoded[index] = value;
            index += 1;
        }
    }
    decoded
}

fn rle_decode_2d(data: Array1<u64>, width: usize, height: usize) -> Array2<u64> {
    let mut decoded = vec![0; width * height];
    // let mut decoded = vec![vec![0; height as usize]; width as usize];
    let mut index = 0;
    for i in (0..data.len()).step_by(2) {
        let count = data[i];
        let value = data[i+1];
        for _ in 0..count {
            decoded[index] = value;
            index += 1;
        }
    }
    Array2::<u64>::from_shape_vec((width, height), decoded).unwrap()
}

#[pyfunction]
fn draw_polygon<'py>(py: Python<'py>,data: PyReadonlyArray2<u64>, points: Vec<Point>) ->  PyResult<PyArray2<u64>> {
    let polygon = Polygon::new(points);
    if !polygon.valid {
        return Err(pyo3::exceptions::PyValueError::new_err("Polygon must be closed"));
    }
    let data = data.as_array().to_owned();
    let mut data = draw_polygon_rs(data, polygon.points.clone());
    return Ok(data.into_pyarray(py));
}

fn draw_polygon_rs(data: Array2<u64>, points: Vec<Point>) -> Array2<u64> {
    let polygon = Polygon::new(points.clone());
    if polygon_out_of_bounds(polygon.points.clone(), data.shape()[0] as usize, data.shape()[1] as usize) {
        panic!("Polygon out of bounds");
    }
    let mut result = data.clone();
    for i in (0..polygon.points.len()).step_by(2) {
        let x = polygon.points[i].x;
        let y = polygon.points[i+1].y;
        result[[x as usize, y as usize]] = 1;
    }
    result
}

fn polygon_out_of_bounds(polygon: Vec<Point>, width: usize, height: usize) -> bool {
    for i in (0..polygon.len()).step_by(2) {
        if out_of_bounds(polygon[i].clone(), width, height) {
            return true;
        }
    }
    false
}

fn get_extents(points: Vec<Point>) -> (i64, i64, i64, i64) {
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    for p in points {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn out_of_bounds(p: Point, width: usize, height: usize) -> bool {
    p.x < 0 || p.x >= (width as i64) || p.y < 0 || p.y >= (height as i64)
}

/// A Python module implemented in Rust.
#[pymodule]
fn upolygon_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rle_encode, m)?)?;
    m.add_function(wrap_pyfunction!(rle_decode, m)?)?;
    m.add_function(wrap_pyfunction!(draw_polygon, m)?)?;
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
        assert_eq!(result, Array1::from(vec![1, 0, 3, 1, 2, 0, 1, 1, 3, 0]));
    }
}