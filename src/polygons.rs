use crate::geometry::*;
use pyo3::prelude::*;
use numpy::ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};

#[pyfunction]
pub fn draw_polygons(py: Python, data: PyReadonlyArray2<u64>, polygons_py: Vec<Vec<Point>>) -> PyResult<Py<PyArray2<u64>>> {
    let mut polygons = Vec::<Polygon>::new();
    for polygon_py in polygons_py {
        let mut polygon = Polygon::new(polygon_py);
        if !polygon.valid() {
            polygon.close();
        }
        polygons.push(polygon);
    }
    let data = data.as_array().to_owned();
    let data = draw_polygons_rs(data, &mut polygons);
    return Ok(data.into_pyarray(py).into_py(py));
}

#[pyfunction]
pub fn draw_polygon(py: Python, data: PyReadonlyArray2<u64>, points_py: Vec<Point>) -> PyResult<Py<PyArray2<u64>>> {
    let mut polygon = Polygon::new(points_py);
    if !polygon.valid() {
        polygon.close();
    }
    let data = data.as_array().to_owned();
    let data = draw_polygon_rs(data, polygon.as_mut());
    return Ok(data.into_pyarray(py).into_py(py));
}

fn get_new_mask(polygons: &mut Vec<Polygon>) -> Array2<u64> {
    let extents = get_furthest_extents(polygons);
    let width = (extents.max_x - extents.min_x) as usize;
    let height = (extents.max_y - extents.min_y) as usize;
    let mask = Array2::<u64>::zeros((width, height));
    polygons.iter_mut().for_each(|polygon| polygon.shift(-extents.min_x, -extents.min_y));
    mask
}

fn get_furthest_extents(polygons: &Vec<Polygon>) -> Extents {
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    for polygon in polygons {
        let extents = polygon.extents();
        if extents.min_x < min_x {
        min_x = extents.min_x;
        }
        if extents.min_y < min_y {
            min_y = extents.min_y;
        }
        if extents.max_x > max_x {
            max_x = extents.max_x;
        }
        if extents.max_y > max_y {
            max_y = extents.max_y;
        }
    }
    Extents {
        min_x,
        min_y,
        max_x,
        max_y,
    }
}

fn draw_polygon_rs(data: Array2<u64>, polygon: &mut Polygon) -> Array2<u64> {
    let width = data.shape()[0] as usize;
    let height = data.shape()[1] as usize;
    let mut data = data.clone();
    let bounded = polygon.clone().out_of_bounds(width, height);
    if bounded {
        data = get_new_mask(&mut vec![polygon.clone()]);
    }
    let points = polygon.points();
    let mut result = data.clone();
    for i in (0..(points.len()-1)) {
        let line = bresenham(points[i], points[i+1]);
        for point in line {
            result[[point.x as usize, point.y as usize]] = 1;
        }
    }
    result
}

fn draw_polygons_rs(data: Array2<u64>, polygons: &mut Vec<Polygon>) -> Array2<u64> {
    let width = data.shape()[0] as usize;
    let height = data.shape()[1] as usize;
    let mut data = data.clone();
    let bounded = polygons.iter().any(|polygon| polygon.clone().out_of_bounds(width, height));
    if bounded {
        data = get_new_mask(polygons);
    }
    for polygon in polygons {
        let points = polygon.points();
        for i in (0..(points.len()-1)) {
            let line = bresenham(points[i], points[i+1]);
            for point in line {
                data[[point.x as usize, point.y as usize]] = 1;
            }
        }
    }
    data
}

fn bresenham(p1: Point, p2: Point) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    let dx = (p2.x - p1.x).abs();
    let dy = (p2.y - p1.y).abs();
    let sx = if p1.x < p2.x { 1 } else { -1 };
    let sy = if p1.y < p2.y { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = p1.x;
    let mut y = p1.y;
    loop {
        points.push(Point::new(x, y));
        if x == p2.x && y == p2.y {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    points
}
