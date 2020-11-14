#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x: x - (width / 2.0),
            y: y - (height / 2.0),
            width,
            height,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x - (width / 2.0);
        self.y = y - (height / 2.0);
        self.width = width;
        self.height = height;
    }

    pub fn contains(&self, rect_other: Rectangle) -> bool {
        let xmin = rect_other.x;
        let xmax = xmin + rect_other.width;

        let ymin = rect_other.y;
        let ymax = ymin + rect_other.height;

        return ((xmin > self.x && xmin < self.x + self.width)
            && (xmax > self.x && xmax < self.x + self.width))
            && ((ymin > self.y && ymin < self.y + self.height)
                && (ymax > self.y && ymax < self.y + self.height));
    }

    pub fn overlaps(&self, rect_other: Rectangle) -> bool {
        return self.x < rect_other.x + rect_other.width
            && self.x + self.width > rect_other.x
            && self.y < rect_other.y + rect_other.height
            && self.y + self.height > rect_other.y;
    }
}

#[cfg(test)]
mod tests {

    use crate::geom::*;
    #[test]
    fn it_works() {
        let rect1 = Rectangle::new(10.0, 10.0, 10.0, 10.0);
        let rect2 = Rectangle::new(11.0, 11.0, 10.0, 10.0);

        assert_eq!(rect1.overlaps(rect2), true);
    }
}
