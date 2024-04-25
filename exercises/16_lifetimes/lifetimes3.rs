// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

///Write-Up
///Il faut déclarer un lifetime sur la structure car la structure n'est pas sauvegardé dans la Heap
///Donc a la fin de l'instance, le pointeur sur la stack est supprimée
///On aurait pu utiliser un type de chaine de caractère qui est déclarée dans la heap au lieu d'un &str

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
