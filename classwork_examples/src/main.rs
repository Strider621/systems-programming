fn main() {
    let freeze = 32;
    let mut farenheit = 62;

    farenheit_to_celsius(farenheit);

    printlm!("{}", farenheit)

   
}

fn farenheit_to_celsius(f: f64) -> f64 {
    f / 32
}

fn celsius_to_farenheit(c: f64) -> f64{

}