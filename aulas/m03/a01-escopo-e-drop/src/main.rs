fn main() {
    let externo = String::from("continua válido");

    {
        let interno = String::from("vive neste bloco");
        println!("Interno: {interno:?}");
    }

    println!("Externo: {externo:?}");
}
