#[derive(Debug, PartialEq,Copy, Clone)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D {
            x,
            y,
        }
    }
}

impl std::ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector2D;

    #[test]
    fn can_create() {
        let v = Vector2D::new(1.0, 1.0);
        assert_eq!(v, Vector2D { x: 1.0, y: 1.0 });
    }

    #[test]
    fn can_add() {
        let v = Vector2D::new(1.0, 1.0);
        assert_eq!(v + v, Vector2D::new(2.0, 2.0));
    }
}
