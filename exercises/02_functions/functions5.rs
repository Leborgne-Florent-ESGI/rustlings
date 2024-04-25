// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

///Write-Up
///La fonction doit retournée un i32
///Le ; a la ligne 16 nécessite l'ajout de ret
///Sinon on peut également supprimer le ; ligne16 car c'est le retour d'un bloc
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
