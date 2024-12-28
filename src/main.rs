fn main() {
    let age: i32 = 26;
    let football: &str = "Gremio";

    if age >= 18 || football == "Inter" {
        println!("Salve");
    } else {
        print!("...");
    }
}