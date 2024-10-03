#[test]

/*
// Fix the errors in-place
fn main() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
       assert_eq!(age, Some(30));
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }
*/


fn main() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30); // Compare to the value 30 directly
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) => println!("age is a new variable, its value is {}", age),
        _ => (),
    }
}

/*
В коде есть небольшая ошибка в ассерте внутри блока if let, где используется assert_eq!(age, Some(30))
Вместо этого нужно сравнивать переменную age из внешней области видимости с числом 30, а не с Some(30).
*/