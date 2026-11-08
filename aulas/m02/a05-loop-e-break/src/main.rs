fn main() {
    let alvo = 5;

    let encontrado = 'busca: loop {
        let mut candidato = 0;

        loop {
            candidato += 1;

            if candidato == alvo {
                break 'busca candidato;
            }
        }
    };

    println!("Encontrado: {encontrado:?}");
}
