mod rle;
mod polygons;
mod geometry;
mod contours;
use crate::polygons::*;
use crate::rle::*;
use crate::contours::*;
use pyo3::{prelude::*};



/// A Python module implemented in Rust.
#[pymodule]
fn upolygon_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rle_encode, m)?)?;
    m.add_function(wrap_pyfunction!(rle_decode, m)?)?;
    m.add_function(wrap_pyfunction!(draw_polygon, m)?)?;
    m.add_function(wrap_pyfunction!(draw_polygons, m)?)?;
    m.add_function(wrap_pyfunction!(find_contours, m)?)?;
    m.add_function(wrap_pyfunction!(draw_complex_polygon, m)?)?;
    Ok(())
}
