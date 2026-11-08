const MILISSEGUNDOS_POR_SEGUNDO: u32 = 1_000;

fn main() {
    let segundos: u32 = 5;
    let total_milissegundos = segundos * MILISSEGUNDOS_POR_SEGUNDO;

    println!("Segundos: {segundos:?}");
    println!("Total em milissegundos: {total_milissegundos:?}");
}
