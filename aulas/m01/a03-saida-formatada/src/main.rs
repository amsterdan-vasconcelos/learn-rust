fn main() {
    let produto = "Caderno";
    let quantidade = 2;
    let preco_centavos = 1290;

    println!("--- Recibo ---");
    println!("Item: {0:?} | quantidade: {1:?}", produto, quantidade);
    println!("Conferência: produto={produto}, quantidade={quantidade}");
    println!("Total em centavos: {preco_centavos}");
}
