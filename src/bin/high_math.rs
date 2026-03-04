fn sinirla(deger: f64) -> f64 {
    deger.clamp(0.0, 100.0)
}

fn main() {
    println!("{}", sinirla(-20.0));
    println!("{}", sinirla(50.0));
    println!("{}", sinirla(200.0));
}
