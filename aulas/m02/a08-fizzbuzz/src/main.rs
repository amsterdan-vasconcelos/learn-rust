fn main() {
    for numero in 1..=15 {
        mostrar_fizz_buzz(numero);
    }
}

fn mostrar_fizz_buzz(numero: u32) {
    if numero % 15 == 0 {
        println!("FizzBuzz");
    } else if numero % 3 == 0 {
        println!("Fizz");
    } else if numero % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{numero:?}");
    }
}
