#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.width, 5);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_is_square() {
        let square = Rectangle::new(5, 5);
        let rectangle = Rectangle::new(5, 10);
        assert!(square.is_square());
        assert!(!rectangle.is_square());
    }
}