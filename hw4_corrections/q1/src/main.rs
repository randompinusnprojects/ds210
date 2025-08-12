use std::ops::Neg;
// use std::any::type_name;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

struct Point<T: Copy + Neg<Output = T>> {
    x: T,
    y: T,
}

impl<T:Copy + Neg<Output = T>> Point<T> {
    
    fn clockwise(&self) -> Point<T> {
        Point { x: self.y, y: -self.x }
    }

    fn counterclockwise(&self) -> Point<T> {
        Point { x: -self.y, y: self.x }
    }
}

fn main() {
    let x: i32 = 1;
    let y: i32 = 0;
    let p = Point{x, y};
    let q = p.clockwise();
    println!("Original          ({}, {})", p.x, p.y);
    println!("Clockwise         ({}, {})", q.x, q.y);
    let x: f32 = 1.0;
    let y: f32 = 0.0;
    let p = Point{x, y};
    let r = p.counterclockwise();
    println!("Original          ({}, {})", p.x, p.y);
    println!("Counterclockwise  ({}, {})", r.x, r.y);
    // println!("{}", type_of(r.x));
}

#[test]
fn testme_int() {
    let p = Point { x: 1, y: 0 };
    let q = p.clockwise();
    let r = p.counterclockwise();
    assert_eq!(q.x, 0);
    assert_eq!(q.y, -1);
    assert_eq!(r.x, 0);
    assert_eq!(r.y, 1);
}

#[test]
fn testme_float() {
    let p = Point { x: 1.0, y: 0.0 };
    let q = p.clockwise();
    let r = p.counterclockwise();
    assert_eq!(q.x, 0.0);
    assert_eq!(q.y, -1.0);
    assert_eq!(r.x, 0.0);
    assert_eq!(r.y, 1.0);
}

