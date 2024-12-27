fn main() {
    let greet: &str = "Hello, Pablo!";

    let dolar_price: f64 = 6.10;
    let euro_price: f64 = 6.20;  // Snake case

    let user_1: &str = "First User";

    let var_with_a_big_name: &str = "Variavel";

    let x = (123, "Pablo");

    // Exemplo de uso das variáveis para evitar warnings
    println!("Greetings: {}", greet);
    println!("Dollar price: ${}\nEuro price: €{}", dolar_price, euro_price);
    println!("User: {}\nBig name variable: {}", user_1, var_with_a_big_name);
    println!("Tuple: {:?}", x);
} //