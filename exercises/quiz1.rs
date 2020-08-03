fn calculate_apple_price(order_amount: i32) -> i32 {
    if order_amount > 40 {
        order_amount * 1
    } else {
        order_amount * 2
    }
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
