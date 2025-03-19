use num_traits::{ToPrimitive, Float};
// both f32 and f64
fn solve2<T:Float>(a:T, b:T) -> f64 {
    let a_f64= a.to_f64().unwrap();
    let b_f64= b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// any type


// just f64
fn solve(a:f64, b:f64) -> f64 {
    let a_f64= a.to_f64().unwrap();
    let b_f64= b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a:f32 = 3.0;
    let b:f32 = 4.0;

    //let a_f64 = a as f64;

    let a_f64= a.to_f64().unwrap();

    println!("{}", solve2::<f32>(a, b));
}
