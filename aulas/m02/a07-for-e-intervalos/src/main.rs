fn main() {
    let medidas = [3, 5, 7];
    let mut soma = 0;

    for medida in medidas {
        soma += medida;
    }

    println!("Soma: {soma:?}");

    for numero in 1..=3 {
        println!("Contagem: {numero:?}");
    }
}
