pub fn basic() {
    let a: i32 = sum(1, 1);
    let b: i32 = sub(3, 1);
    let c: i32 = mul(3, 1);
    let d: i32 = div(3, 1);
    println!("Resultado de 1 + 1 é {}", a);
    println!("Resultado de 3 - 1 é {}", b);
    println!("Resultado de 3 * 1 é {}", c);
    println!("Resultado de 3 / 1 é {}", d);
}

fn sum(a: i32, b: i32) -> i32 {
    let result: i32 = a + b;
    result
}

fn sub(a: i32, b: i32) -> i32 {
    let result: i32 = a - b;
    result
}

fn mul(a: i32, b: i32) -> i32 {
    let result: i32 = a * b;
    result
}

fn div(a: i32, b: i32) -> i32 {
    let result: i32 = a / b;
    result
}