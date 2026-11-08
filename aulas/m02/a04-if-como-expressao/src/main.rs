fn main() {
    let nota: i32 = 82;

    let conceito = if nota >= 90 {
        "A"
    } else if nota >= 80 {
        "B"
    } else {
        "C ou menos"
    };

    println!("Nota: {nota:?}");
    println!("Conceito: {conceito:?}");
}
