fn main() {
    let total: i32 = 10 + 2 * 4;
    let com_parenteses: i32 = (10 + 2) * 4;

    let dentro_do_limite = total <= 20;
    let liberado = dentro_do_limite && total != 0;
    let tem_excecao = total == 14 || total == 18;

    println!("Total: {total:?}");
    println!("Com parênteses: {com_parenteses:?}");
    println!("Dentro do limite: {dentro_do_limite:?}");
    println!("Liberado: {liberado:?}");
    println!("Tem exceção: {tem_excecao:?}");
}
