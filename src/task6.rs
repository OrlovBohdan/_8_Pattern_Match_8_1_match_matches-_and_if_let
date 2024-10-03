#[test]

/*
fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
}
*/


fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

/*
Использование if let Some(i) = o позволяет проверить, содержит ли o значение Some(i).
Если это так, то переменная i получает значение, и выполняются все операции внутри блока.
Если o будет None, код внутри if let не выполнится, что соответствует поведению изначального match блока с _.
*/