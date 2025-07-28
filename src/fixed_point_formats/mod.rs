// Q7 is a special struct we've made that can be used to represent a
// fixed-point decimal number between 1.0 and -1.0
// The last bit denotes the sign, the other 7 bits denotes the decimals
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

// It is designed as a compact fixed point number storage type, so it
// should be able to convert to / from an f64
impl From<f64> for Q7 {
    fn from(value: f64) -> Self {
        // n >= -1.0 && n <= 1.0
        if value >= 1.0 {
            Q7(127) // Clamp to 7 bit max value
        } else if value <= -1.0 {
            Q7(-128) // Clamp to 7 bit min value
        } else {
            Q7((value * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        (value.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(value: f32) -> Self {
        Q7::from(value as f64)
    }
}

impl From<Q7> for f32 {
    fn from(value: Q7) -> Self {
        f64::from(value) as f32
    }
}

pub fn run() {
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7::from(1.0));
        assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }
}