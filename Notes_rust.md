# Notes rust 

Ce document contient des brièves notes sur le fonctionnement de Rust. 

##  Ownership 

Rust utilise la notion de "Stack" et de "Heap". Stack fait référence à l'image d'une pile d'assiette tandis que le Heap fait réfrence à l'allocation de mémoire (image du serveur qui donne une "adresse" à chaque client).

Pour copier une variable qui n'est pas dans les data type simples, il suffit d'utiliser la fonction "clone()" qui permet de faire une deep copy. 

Dans Rust, il y a des "move" dès lors qu'une variable est bougé d'un scope à un autre (d'une fonction à une autre).

Pour éviter de donner la propriété à une fonction, il suffit donc de faire référence à la variable à l'aide de "&" qui permet de faire seulement référence à la variable (notion de borrowing).

```Rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Ainsi la référence est immutable, il est impossible de modifier avec seulement &. Il est possible d'utiliser une "mutable reference" : &mut. Néanmoins, il faut garder en tête que cette notion est n'utilisable qu'un fois. Il est impossible faire une référence mutable à une référence mutable.

``` Rust 
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

```

Ce morceau de code ne fonctionne donc pas. Ce comportement vise à éviter les "data race" souvent difficile à débug. Il est aussi impossible de combiner les références mutables et immutables.  


