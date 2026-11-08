fn main() {
    let minutos: u32 = 5;
    let segundos_por_minuto = 60;
    let total_segundos = minutos * segundos_por_minuto;

    let temperatura_celsius: i16 = -5;
    let temperatura_precisa: f64 = -5.5;

    println!("Minutos: {minutos:?}");
    println!("Segundos: {total_segundos:?}");
    println!("Temperatura inteira: {temperatura_celsius:?}");
    println!("Temperatura decimal: {temperatura_precisa:?}");
}
