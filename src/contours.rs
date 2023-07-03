use crate::geometry::*;
use pyo3::prelude::*;
use numpy::ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};

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


#[cfg(test)]
mod tests {
    use ndarray::{Array2};
    #[test]
    fn test_find_contours_empty() {
        let image = Array2::<u8>::zeros((10, 10));
        let (labels, outer_paths, inner_paths) = super::find_contours_rs(image);
        assert_eq!(labels, Array2::<i8>::zeros((10, 10)));
        assert_eq!(outer_paths, Vec::<Vec<_>>::new());
        assert_eq!(inner_paths, Vec::<Vec<_>>::new());
    }
    #[test]
    fn test_find_contours_single_outer() {
        let mut image = Array2::<u8>::zeros((10, 10));
        image[[1, 1]] = 1;
        image[[1, 2]] = 1;
        image[[1, 3]] = 1;
        image[[2, 1]] = 1;
        image[[2, 3]] = 1;
        image[[3, 1]] = 1;
        image[[3, 2]] = 1;
        image[[3, 3]] = 1;
        let (labels, outer_paths, inner_paths) = super::find_contours_rs(image);
        assert_eq!(labels, Array2::<i8>::zeros((10, 10)));
    }
}