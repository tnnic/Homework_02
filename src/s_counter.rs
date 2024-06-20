#![allow(dead_code)]
pub type SignedCounter = isize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_default_signed_counter(){
        assert_eq!(default_signed_counter(),0);
    }
    #[test]
    fn test_prev_signed(){
        assert_eq!(prev_signed(5),4);
    }
    #[test]
    fn test_next_signed(){
        assert_eq!(next_signed(5), 6);
    }
}
