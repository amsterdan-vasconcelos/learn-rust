fn main() {
    let soma = somar(4, 7);
    let diferenca = subtrair(9, 3);

    println!("Soma: {soma:?}");
    println!("Diferença: {diferenca:?}");
}

fn somar(a: i32, b: i32) -> i32 {
    a + b
}

fn subtrair(a: i32, b: i32) -> i32 {
    return a - b;
}
