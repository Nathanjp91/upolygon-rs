use crate::geometry::*;
use pyo3::prelude::*;
use numpy::ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use std::ops::{Add, Index, IndexMut};

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


enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

impl Direction {
    fn to_index(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::DownRight => 1,
            Direction::Down => 2,
            Direction::DownLeft => 3,
            Direction::Left => 4,
            Direction::UpLeft => 5,
            Direction::Up => 6,
            Direction::UpRight => 7,
        }
    }
    fn from_index(index: usize) -> Direction {
        match index {
            0 => Direction::Right,
            1 => Direction::DownRight,
            2 => Direction::Down,
            3 => Direction::DownLeft,
            4 => Direction::Left,
            5 => Direction::UpLeft,
            6 => Direction::Up,
            7 => Direction::UpRight,
            _ => panic!("Invalid index"),
        }
    }
    fn to_point(&self) -> Point {
        match self {
            Direction::Right => Point::new(1, 0),
            Direction::DownRight => Point::new(1, 1),
            Direction::Down => Point::new(0, 1),
            Direction::DownLeft => Point::new(-1, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::UpLeft => Point::new(-1, -1),
            Direction::Up => Point::new(0, -1),
            Direction::UpRight => Point::new(1, -1),
        }
    }
    fn iter() -> impl Iterator<Item = Direction> {
        vec![
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
        ]
        .into_iter()
    }
    fn iter_from(index: usize) -> impl Iterator<Item = Direction> {
        let mut directions = Vec::<Direction>::new();
        for i in index..8 {
            directions.push(Direction::from_index(i));
        }
        for i in 0..index {
            directions.push(Direction::from_index(i));
        }
        directions.into_iter()
    }
    fn iter_from_direction(direction: Direction) -> impl Iterator<Item = Direction> {
        let index = direction.to_index();
        Self::iter_from(index)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

fn tracer(
    p_point: Point,
    old_direction: Direction,
    n_point: &mut Point,
    image: ArrayView2<u8>,
    labels: &mut ArrayViewMut2<i8>,
) -> Direction {
    *n_point = p_point.clone().to_owned();
    for direction in Direction::iter_from_direction(old_direction) {
        let tmp_point = direction.to_point() + p_point;
        let tmpx = tmp_point.x as usize;
        let tmpy = tmp_point.y as usize;
        if image[[tmpy, tmpx]] == 1 {
            *n_point = tmp_point.clone().to_owned();
            return direction;
        } else {
            labels[[tmpy as usize, tmpx as usize]] = -1;
        }
    }
    unreachable!(); // Indicate that the loop is guaranteed to terminate
}

fn contour_trace(
    p_point: Point,
    c: i8,
    image: ArrayView2<u8>,
    labels: &mut ArrayViewMut2<i8>,
    inner: bool,
) -> Vec<Point> {
    let starting_point = p_point.clone();
    let mut n_point = Point::new(0,0);
    let mut direction = if inner { Direction::DownRight } else { Direction::UpLeft };
    let mut last_point_was_s = false;
    let mut path = vec![p_point + Direction::UpLeft.to_point()];
    direction = tracer(p_point, direction, &mut n_point, image, labels);
    let t_point = n_point.clone().to_owned();

    if t_point == starting_point {
        return path;
    }
    path.push(t_point.clone() + Direction::UpLeft.to_point());

    labels[n_point] = c;
    loop {
        direction = tracer(p_point, direction, &mut n_point, image, labels);
        if last_point_was_s && n_point == t_point {
            return path;
        }
        path.push(n_point.clone() + Direction::UpLeft.to_point());

        labels[n_point] = c;
        last_point_was_s = n_point == starting_point
    }
}

impl Index<Point> for Array2<u8> {
    type Output = u8;
    fn index(&self, index: Point) -> &u8 {
        &self[[index.y as usize, index.x as usize]]
    }
}
impl IndexMut<Point> for Array2<i8> {
    fn index_mut(&mut self, index: Point) -> &mut i8 {
        &mut self[[index.y as usize, index.x as usize]]
    }
}
impl Index<Point> for Array2<i8> {
    type Output = i8;
    fn index(&self, index: Point) -> &i8 {
        &self[[index.y as usize, index.x as usize]]
    }
}
impl Index<Point> for ArrayView2<'_, u8> {
    type Output = u8;
    fn index(&self, index: Point) -> &u8 {
        &self[[index.y as usize, index.x as usize]]
    }
}
impl Index<Point> for ArrayViewMut2<'_, i8> {
    type Output = i8;
    fn index(&self, index: Point) -> &i8 {
        &self[[index.y as usize, index.x as usize]]
    }
}
impl IndexMut<Point> for ArrayViewMut2<'_, i8> {
    fn index_mut(&mut self, index: Point) -> &mut i8 {
        &mut self[[index.y as usize, index.x as usize]]
    }
}

fn find_contours_rs(
    image: Array2<u8>,
) -> (Array2<i8>, Vec<Vec<Point>>, Vec<Vec<Point>>) {
    let mut c = i8::default();
    let (height, width) = image.dim();
    let mut padded_image = Array2::<u8>::zeros((height + 2, width + 2));
    let mut labels = Array2::<i8>::zeros((height + 2, width + 2));
    let mut inner_paths = Vec::new();
    let mut outer_paths = Vec::new();

    padded_image.slice_mut(s![1..height+1, 1..width+1]).assign(&image);

    for y in 1..height {
        for x in 1..width {
            if padded_image[[y, x]] == 0 { continue; }

            let mut handled = false;
            let point = Point::new(x as i64, y as i64);
            if labels[[y, x]] == 0 && padded_image[[y-1, x]] == 0 {
                labels[[y, x]] = c;
                let path = contour_trace(point, c, padded_image.view(), &mut labels.view_mut(), false);
                outer_paths.push(path);
                c += 1;
                handled = true;
            }

            if labels[[y+1, x]] != -1 && image[[y+1, x]] == 0 {
                let path = if labels[[y, x]] == 0 {
                    contour_trace(point, labels[[y,x-1]], padded_image.view(), &mut labels.view_mut(), true)
                } else {
                    contour_trace(point, labels[[y,x]], padded_image.view(), &mut labels.view_mut(), true)
                };
                inner_paths.push(path);
                handled = true;
            }

            if !handled && labels[[y, x]] == 0{
                labels[[y, x]] = labels[[y, x-1]];
            }
        }
    }

    (labels.slice(s![1..height+1, 1..width+1]).to_owned(), outer_paths, inner_paths)
}

#[pyfunction]
pub fn find_contours(py: Python, image: PyReadonlyArray2<u8>) -> PyResult<(Py<PyArray2<i8>>, Vec<Vec<i64>>, Vec<Vec<i64>>)> {
    let image = image.as_array();
    let (labels, outer_paths, inner_paths) = find_contours_rs(image.to_owned());
    let outer_paths = outer_paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|point| point.to_tuple()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let inner_paths = inner_paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|point| point.to_tuple()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok((labels.into_pyarray(py).into_py(py), outer_paths, inner_paths))
}