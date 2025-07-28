// This takes a byte (random) and give you it as a fraction of it's max value
// between 0 and 1 - a way of getting randomness
fn mock_rand(n: u8) -> f32 {
    (n as f32) / 255.0
}

// but division is an expensive operation for CPUs. Can we do it
// another way?

// The floating point logic implemented in floating_points has an f32 representing
// a floating point number. The mantissa / significand of this number is a number
// between 0 and 1, so why dont convert it and use that.
fn custom_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * ( m - 0.5)
}

pub fn run() {
    println!("Max of input range: {:08b} -> {:?}", 0xff, custom_rand(0xff));
    println!("Mid of input range: {:08b} -> {:?}", 0x7f, custom_rand(0x7f));
    println!("Min of input range: {:08b} -> {:?}", 0x00, custom_rand(0x00));
}