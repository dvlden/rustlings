fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let e = f64::EPSILON;

    if (y - x).abs() > e {
        println!("Success!");
    }
}
