use crate::osm::model::coordinate::Coordinate;

#[derive(Debug, Default, Clone)]
pub struct BoundingBox{
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl BoundingBox {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> BoundingBox {
        BoundingBox {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn merge_point(&mut self, coordinate: &Coordinate) {
        if coordinate.lon < self.left {
            self.left = coordinate.lon;
        }

        if coordinate.lon > self.right {
            self.right = coordinate.lon;
        }

        if coordinate.lat > self.top {
            self.top = coordinate.lat;
        }

        if coordinate.lat < self.bottom {
            self.bottom = coordinate.lat;
        }
    }

    pub fn merge_bounding_box(&mut self, other: &BoundingBox) {
        if other.left < self.left {
            self.left = other.left;
        }

        if other.right > self.right {
            self.right = other.right;
        }

        if other.top > self.top {
            self.top = other.top;
        }

        if other.bottom < self.bottom {
            self.bottom = other.bottom;
        }
    }
}
