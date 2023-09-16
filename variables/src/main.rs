fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    const MAX_POINTS: u32 = 100_000;

    let _x = 10;

    let _x = 505;

    let _guess: u32 = "42"
        .parse()
        .expect("숫자가 아닌데?");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let _a: i8 = -31;
    let _b: u8 = MAX_POINTS as u8;
    let _c = 33i64;
    let _d = 127i8;

    let _hex = 0xff;
    let _oct = 0o77;
    let _bin = 0b1111_0000;
    let _byte = b'A';
}
