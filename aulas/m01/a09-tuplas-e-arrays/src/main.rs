fn main() {
    let item = ("Caderno", 2, true);
    let (produto, quantidade, disponivel) = item;

    let precos_centavos: [i32; 3] = [1290, 450, 700];
    let segundo_preco = precos_centavos[1];

    println!("Produto: {produto:?}");
    println!("Quantidade: {quantidade:?}");
    println!("Disponível: {disponivel:?}");
    println!("Segundo preço: {segundo_preco:?}");
    println!("{precos_centavos:?}")
}
