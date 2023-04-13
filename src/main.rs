fn cast(x: f32) -> u8 {
    x as u8
}

fn main() {
    let too_big = 300.0;
    let too_small = -100.0;
    let nan = f32::NAN;

    println!("too_big_casted = {}", cast(too_big));
    println!("too_small_casted = {}", cast(too_small));
    println!("not_a_number_casted = {}", cast(nan));

    let x: f32 = 300.0;
    let y: u8 = unsafe { x.to_int_unchecked() };
    println!("y = {}", y);
}
