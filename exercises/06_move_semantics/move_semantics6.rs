// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

///Write-up
///J'ai modifier la fonction get_char et son appel afin que elle ne s'approprie pas la variable.
///En utilisant le pointeur, rust ne donne pas le ownership à get_char.
///Ensuite j'ai enlevé tout les '&' dans les référencez de la fonction string_uppercase
///Etant la dernière fonction appellé, elle peut s'approprier data

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
