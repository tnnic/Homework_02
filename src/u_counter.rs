
pub type UnsignedCounter = usize;

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_default_unsigned_counter(){
        assert_eq!(default_unsigned_counter(), 0);
    }
    
    #[test]
    fn test_next_unsigned(){
        assert_eq!(next_unsigned(5), 6);
    }
}
