use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Bounds<T> {
    lower: T,
    upper: T
}

impl Into<Bounds<Vec2>> for &Window {
    fn into(self) -> Bounds<Vec2> {
        let (width, height) = (self.width() / 2.0, self.height() / 2.0);
        Bounds {
            lower: Vec2::new(-width, -height),
            upper: Vec2::new(width, height)
        }
    }
}

pub trait BoundTo {
    type Item;
    fn bound_to(self, center: Self::Item, bounds: Bounds<Self::Item>) -> Self::Item;
}

impl BoundTo for Vec2 {
    type Item = Vec2;

    fn bound_to(self, center: Vec2, bounds: Bounds<Vec2>) -> Vec2 {
        let mut new = Vec2::new(self.x, self.y) + center;

        if new.x < bounds.lower.x {
            new.x = new.x + bounds.upper.x - bounds.lower.x;
        } else if new.x > bounds.upper.x {
            new.x = new.x - bounds.lower.x + bounds.upper.x;
        }

        if new.y < bounds.lower.y {
            new.y = new.y + bounds.upper.y - bounds.lower.y;
        } else if new.y > bounds.upper.y {
            new.y = new.y - bounds.upper.y + bounds.lower.y;
        }

        new
    }
}
