fn main() {
    mostrar_soma(4, 7);
}

fn mostrar_soma(primeiro: i32, segundo: i32) {
    let soma = primeiro + segundo;
    println!("Soma: {soma:?}");
}
