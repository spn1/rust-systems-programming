const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

pub fn run() {
    let number: f32 = 42.42;

    let (sign, exp, frac) = to_parts(number);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", number, n_);
    println!("field \t\t| as bits \t| as real number");
    println!("sign \t\t| {:08b} \t| {}", sign, sign_);
    println!("exponent \t| {:08b} \t| {}", exp, exp_);
    println!("mantissa \t| {:23b} \t| {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign= (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(
    sign: u32,
    exponent: u32,
    fraction: u32
) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    let mut mantissa = 0.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;

        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

fn from_parts(
    sign: f32,
    exponent: f32,
    mantissa: f32,
) -> f32 {
    sign * exponent * mantissa
}

/** Old */

pub fn decode_old(number: f32) {
    //42.42 in bits is 0_10000100_01010011010111000010100
    // Isolating the sign_bit (0)

    // Convert number to u32, to keep the bit pattern.
    let n_bits: u32 = number.to_bits();

    // Right shift by 31 to get the sign (31 is 4 bytes - 1)
    let sign_bit = n_bits >> 31;

    // Isolating the exponent (10000100)
    let exponent = n_bits >> 23; // Remove the significand
    let exponent = exponent & 0xff; // Exclude the sign bit
    let exponent = (exponent as i32) - 127; // Interpret the remainder as a signed integer and subtract the bias (127)

    // Isolating the significand
    let mut significand: f32 = 1.0;

    for i in 0..23 {
        // Create a mask at ...0001 and shift it left once iteration
        let mask = 1 << i;

        // Check if there is a bit set at the mask position in the full number
        let one_at_bit_1 = n_bits & mask;

        if one_at_bit_1 != 0 {
            // If there is, we need to calculate what fraction that bit represends and
            // add it to the significand total.
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            significand += weight;
        }
    }

    println!("{} to parts:", number);
    println!("sign: {}", sign_bit);
    println!("exponent: {}", exponent);
    println!("significand: {}", significand);
}