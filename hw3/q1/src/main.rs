use core::f32::consts::PI;

fn main() {
    // Test for Sphere
    let r:f32 = -2.0;
    let mut s1 = Shape::new(3, &[r]);
    let b = Shape::verify(3, &[r]);
    let a = s1.area();
    let v = s1.volume();
    s1.doubletall();
    println!("{}, {}, {:?}, {}", a, v, s1, b);

    // Test for Cuboid
    let l:f32 = 1.0;
    let w:f32 = 10.0;
    let h:f32 = 1.0;
    let b2 = Shape::verify(2, &[l, w, h]);
    let mut c1 = Shape::new(2, &[l, w, h]);
    let a2 = c1.area();
    let v2 = c1.volume();
    c1.doubletall();
    println!("{}, {}, {:?}, {}", a2, v2, c1, b2);

    // Test for RectPy
    let l2:f32 = 5.0;
    let w2:f32 = 10.0;
    let h2:f32 = 2.0;
    let b3 = Shape::verify(1, &[l2, w2, h2]);
    let mut r1 = Shape::new(1, &[l2, w2, h2]);
    let a3 = r1.area();
    let v3 = r1.volume();
    r1.doubletall();
    println!("{}, {}, {:?}, {}", a3, v3, r1, b3);
    
}

#[derive(Debug)]
enum Shape {
    RectPy {l: f32, w: f32, h: f32,},
    Cuboid {l: f32, w: f32, h: f32,},
    Sphere {r: f32,},
}

impl Shape { // think about validation method
    fn new(t:u8, arr:&[f32]) -> Shape {
        match t {
            1 if Shape::verify(t, arr) => Shape::RectPy {l: arr[0], w: arr[1], h: arr[2]},
            2 if Shape::verify(t, arr) => Shape::Cuboid {l: arr[0], w: arr[1], h: arr[2]},
            3 if Shape::verify(t, arr) => Shape::Sphere {r: arr[0]},
            _ => todo!("Invalid value"),
        }
    }

    fn verify(t:u8, arr:&[f32]) -> bool {
        match t {
            1 => arr.len() == 3 && arr.iter().all(|&x| x>0.0),
            2 => arr.len() == 3 && arr.iter().all(|&x| x>0.0),
            3 => arr.len() == 1 && arr[0] > 0.0,
            _ => false
        }
    }

    fn area(&self) -> f32 {
        match self {
            Shape::RectPy{l, w, h} => l * w + l*(w/2.0 * w/2.0 + h * h).sqrt() + w *(l/2.0 * l/2.0 + h * h).sqrt(),
            Shape::Cuboid{l, w, h} => 2.0 * l * w + 2.0 * w * h + 2.0 * l * h,
            Shape::Sphere{r} => 4.0 * PI * r * r,
        }
    }

    fn volume(&self) -> f32 {
        match self {
            Shape::RectPy{l, w, h} => l * w * h * 1.0/3.0,
            Shape::Cuboid{l, w, h} => l * w * h,
            Shape::Sphere{r} => 4.0 / 3.0 * PI * r * r * r,
        }
    }

    fn doubletall(&mut self) {
        match self {
            Shape::RectPy{h, ..} | Shape::Cuboid{h, ..} => *h = 2.0 * (*h),
            Shape::Sphere{r} => *r = 2.0 * (*r),
        }
    }
}