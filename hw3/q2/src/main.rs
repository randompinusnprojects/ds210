use core::f32::consts::PI;
fn main() {
    let polyradii:[f32;3] = [10.0, 50.0, 200.0];
    let polysides:[u32;10] = [4, 8, 16, 32, 64, 128, 256, 512, 2048, 65536];
    for i in 0..polyradii.len() {
        println!("Comparison when radius of side: {}", polyradii[i]);
        for j in 0..polysides.len() {
            let poly = Regpol{s: polysides[j], l: revert(polyradii[i], polysides[j])};
            let ic:f32 = poly.apothem();
            let cc:f32 = polyradii[i];
            let ic_area:f32 = ic * ic * PI;
            let cc_area:f32 = cc * cc * PI;
            let q1:f32 = ic_area / poly.area();
            let q2:f32 = poly.area() / cc_area;
            println!("polygon with {} sides", polysides[j]);
            println!("ratio of Inscribed / Polygon: {}, ratio of Polygon / Circumscribed: {}", q1, q2);
        }
    }
}

struct Regpol {
    s: u32,
    l: f32,
}

trait Calculator {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn radius(&self) -> f32;
    fn apothem(&self) -> f32;
}

impl Calculator for Regpol {
    fn area(&self) -> f32 {
        return (1.0/2.0) * self.apothem() * self.perimeter();
    }

    fn perimeter(&self) -> f32 {
        return self.l * self.s as f32;
    }

    fn radius(&self) -> f32 {
        return self.l / (2.0 * (PI/self.s as f32).sin());
    }

    fn apothem(&self) -> f32 {
        return self.l / (2.0 * (PI / self.s as f32).tan());
    }
}

fn revert(r:f32, s:u32) -> f32 {
    return r * (2.0 * (PI/s as f32).sin());
}