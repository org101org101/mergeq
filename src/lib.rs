pub fn add(left: u64, right: u64) -> u64 {
    left + right + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(5, add(2, 2));
    }
}

// fake update here