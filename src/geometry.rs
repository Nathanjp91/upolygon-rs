use pyo3::{prelude::*};


#[derive(FromPyObject, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn to_tuple(&self) -> Vec<i64> {
        vec![self.x, self.y]
    }
    pub fn shift(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }
    pub fn shift_clone(&self, x: i64, y: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
    pub fn out_of_bounds(self, width: usize, height: usize) -> bool {
        self.x < 0 || self.y < 0 || self.x >= width as i64 || self.y >= height as i64
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    pub fn perpendicular_distance_to_line(self, line: &Line) -> f64 {
        let x1 = line.start.x as f64;
        let y1 = line.start.y as f64;
        let x2 = line.end.x as f64;
        let y2 = line.end.y as f64;
        let x3 = self.x as f64;
        let y3 = self.y as f64;
        let px = x2 - x1;
        let py = y2 - y1;
        let d_squared = px * px + py * py;
        let u = ((x3 - x1) * px + (y3 - y1) * py) / d_squared;
        let mut x = x1;
        let mut y = y1;
        if u > 1.0 {
            x = x2;
            y = y2;
        } else if u > 0.0 {
            x += px * u;
            y += py * u;
        }
        let dx = x - x3;
        let dy = y - y3;
        (dx * dx + dy * dy).sqrt()
    }
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Point>,
    extents: Extents,
    valid: bool,
    shifted: bool,
    offset: Point,
}
#[derive(Debug, Clone, Copy)]
pub struct Extents {
    pub min_x: i64,
    pub min_y: i64,
    pub max_x: i64,
    pub max_y: i64,
}

pub struct ComplexPolygon {
    polygons: Vec<Polygon>,
    extents: Extents,
    valid: bool,
    shifted: bool,
    offset: Point,
}

impl ComplexPolygon {
    pub fn from_paths(paths: Vec<Vec<Point>>) -> Self {
        let mut polygons = Vec::<Polygon>::new();
        let mut extents = Extents {
            min_x: std::i64::MAX,
            min_y: std::i64::MAX,
            max_x: std::i64::MIN,
            max_y: std::i64::MIN,
        };
        let mut valid = true;
        for path in paths {
            let mut polygon = Polygon::new(path);
            if !polygon.valid {
                valid = false;
                polygon.close();
            }
            if polygon.extents.min_x < extents.min_x {
                extents.min_x = polygon.extents.min_x;
            }
            if polygon.extents.min_y < extents.min_y {
                extents.min_y = polygon.extents.min_y;
            }
            if polygon.extents.max_x > extents.max_x {
                extents.max_x = polygon.extents.max_x;
            }
            if polygon.extents.max_y > extents.max_y {
                extents.max_y = polygon.extents.max_y;
            }
            polygons.push(polygon);
        }
        Self {
            polygons,
            extents,
            valid,
            shifted: false,
            offset: Point { x: 0, y: 0 },
        }
    }
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        let extents = Polygon::get_extents(points.as_ref());
        let valid = points[0].x == points[points.len() - 1].x && points[0].y == points[points.len() - 1].y;
        Self {
            points,
            extents,
            valid,
            shifted: false,
            offset: Point { x: 0, y: 0 },
        }
    }
    pub fn close(&mut self) {
        if self.points.len() > 0 {
            if self.points[0] != self.points[self.points.len() - 1] {
                self.points.push(self.points[0].clone());
                self.valid = true;
            }
        }
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    pub fn points_as_mut(&mut self) -> &mut Vec<Point> {
        &mut self.points
    }
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
    pub fn valid(&self) -> bool {
        self.valid
    }
    pub fn extents(&self) -> Extents {
        self.extents.clone()
    }
    fn get_extents(points: &Vec<Point>) -> Extents {
        let mut min_x = std::i64::MAX;
        let mut min_y = std::i64::MAX;
        let mut max_x = std::i64::MIN;
        let mut max_y = std::i64::MIN;
        for point in points.iter() {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
        Extents {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
    pub fn shift(&mut self, x: i64, y: i64) {
        for point in self.points.iter_mut() {
            point.x += x;
            point.y += y;
        }
        self.extents.min_x += x;
        self.extents.min_y += y;
        self.extents.max_x += x;
        self.extents.max_y += y;
        self.offset.x += x;
        self.offset.y += y;
        self.shifted = true;
        if (self.offset.x == 0) && (self.offset.y == 0) {
            self.shifted = false;
        }
    }
    pub fn correct(&mut self) {
        if self.shifted {
            self.shift(-self.offset.x, -self.offset.y);
        }
    }
    pub fn out_of_bounds(self, width: usize, height: usize) -> bool {
        self.extents.min_x < 0 || self.extents.min_y < 0 || self.extents.max_x >= width as i64 || self.extents.max_y >= height as i64
    }
}