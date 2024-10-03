#[test]

/*

// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    __ {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
*/


// Fill in the blank



fn main() {
    let a = Foo::Bar(1);

    match a {
        Foo::Bar(i) => {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
}
enum Foo {
    Bar(u8),
}
