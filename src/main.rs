fn main() {
    let age: i32 = 26;
    let mut result: bool = age > 18;
    result: bool = age < 18;
    result: bool = age >= 18;
    result: bool = age == 18;
    result: bool = age != 18;

    if result > 18 {
        println!("Qual cerveja você deseja?");
    } else {
        println!("Não podemos vender para menor");
    }
}