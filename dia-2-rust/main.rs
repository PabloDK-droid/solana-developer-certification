fn imprimir(s: &String) {
    println!("{}", s);
}

fn agregar(s: &mut String) {
    s.push_str(" Developer");
}

fn main() {
    let mut nombre = String::from("Solana");
    imprimir(&nombre);
    agregar(&mut nombre);
    println!("{}", nombre);
}
