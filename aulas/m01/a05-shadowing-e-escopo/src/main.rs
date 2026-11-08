fn main() {
    let valor = 5;
    let valor = valor + 1;

    {
        let valor = valor * 2;
        println!("Dentro do bloco: {valor:?}");
    }

    println!("Fora do bloco: {valor:?}");

    let mut contador = 0;
    contador += 1;
    println!("Contador: {contador:?}");
}
