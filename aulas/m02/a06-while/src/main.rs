fn main() {
    let mut contagem: i32 = 5;

    while contagem > 0 {
        println!("{contagem:?}");
        contagem -= 1;
    }

    println!("Fim!");
}
