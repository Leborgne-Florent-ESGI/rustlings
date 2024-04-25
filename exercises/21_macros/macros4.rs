// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

///Write-Up
///Chaque règle de correspondance contenu dans une macro doivent être séparé par des points virgules.
///Cela permet à rust d'identifier clairement la correspondance lors de l'appel de la macro.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
