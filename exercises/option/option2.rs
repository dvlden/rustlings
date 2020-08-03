fn main() {
    let optional_value = Some(String::from("rustlings"));

    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    while let Some(value) = optional_values_vec.pop() {
        if let Some(value) = value {
            println!("current value: {}", value);
        }
    }
}
