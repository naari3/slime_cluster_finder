pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    pub fn new(seed: i64) -> JavaRandom {
        let seed = (seed ^ 0x5DEECE66D) & 0xFFFFFFFFFFFF;
        JavaRandom { seed }
    }

    fn next(&mut self, bits: i32) -> i32 {
        self.seed = (self.seed.wrapping_mul(0x5DEECE66D) + 0xB) & 0xFFFFFFFFFFFF;
        (self.seed >> (48 - bits)) as _
    }

    pub fn next_int(&mut self, bound: i32) -> i32 {
        let mut r = self.next(31);
        let m = bound - 1;

        if (bound & m) == 0 {
            r = ((bound as i64 * r as i64) >> 31) as i32;
        } else {
            let mut u = r;
            r = u % bound;
            while u.wrapping_sub(r) + m < 0 {
                u = self.next(31);
            }
        };
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_random() {
        let mut random = JavaRandom::new(0);
        assert_eq!(random.next_int(100), 60);
        assert_eq!(random.next_int(100), 48);
        assert_eq!(random.next_int(100), 29);
        assert_eq!(random.next_int(100), 47);
    }
}
