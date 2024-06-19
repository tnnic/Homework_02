pub mod types {
    pub const VEC3_LEN: usize = 3;

    pub type Pair = (i32, i32);
    pub type Vec3 = [i32; VEC3_LEN];
    pub type SignedCounter = isize;
    pub type UnsignedCounter = usize;
}
mod utils {
    use crate::types::*;

    pub mod s_counter {
        use super::*;

        pub fn default_signed_counter() -> SignedCounter {
            0
        }

        pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
            counter - 1
        }

        pub fn next_signed(counter: SignedCounter) -> SignedCounter {
            counter + 1
        }
    }
    pub mod u_counter {
        use super::*;

        pub fn default_unsigned_counter() -> UnsignedCounter {
            0
        }

        pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
            counter + 1
        }
    }
    pub mod vectors {
        use super::*;
        pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
            let mut c = default_vec3();
            for i in 0..3 {
                c[i] = a[i] + b[i];
            }
            c
        }

        pub fn default_vec3() -> Vec3 {
            [0; 3]
        }

        pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
            let mut c = 0;
            for i in 0..VEC3_LEN {
                c += a[i] + b[i];
            }
            c
        }
    }
    pub mod pairs {
        use super::*;
        pub fn default_pair() -> Pair {
            (0, 0)
        }

        pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
            (a.0 + b.0, a.1 + b.1)
        }

        pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
            a.0 + a.1 + b.0 + b.1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::types::*;
    use super::utils::*;

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(s_counter::default_signed_counter(), 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(u_counter::default_unsigned_counter(), 0);
    }

    #[test]
    fn test_next_signed() {
        assert_eq!(s_counter::next_signed(5), 6);
    }

    #[test]
    fn test_next_unsigned() {
        assert_eq!(u_counter::next_unsigned(5), 6);
    }

    #[test]
    fn test_prev_signed() {
        assert_eq!(s_counter::prev_signed(5), 4);
    }

    #[test]
    fn test_default_vec3() {
        assert_eq!(vectors::default_vec3(), [0; 3]);
    }

    #[test]
    fn test_vec3_vector_sum() {
        assert_eq!(vectors::vec3_vector_sum([1, 2, 3], [4, 5, 6]), [5, 7, 9]);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        assert_eq!(vectors::vec3_scalar_sum([1, 2, 3], [4, 5, 6]), 21);
    }

    #[test]
    fn test_default_pair() {
        assert_eq!(pairs::default_pair(), (0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        assert_eq!(pairs::pair_vector_sum((1, 2), (3, 4)), (4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        assert_eq!(pairs::pair_scalar_sum((1, 2), (3, 4)), 10);
    }
}
