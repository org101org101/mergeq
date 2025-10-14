pub fn add(left: u64, right: u64) -> u64 {
    left + right + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, 3 + 2);
    }
}

// fake update here