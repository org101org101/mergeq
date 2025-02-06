pub fn plus(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

pub fn mul(left: u64, right: u64) -> u64 {
    left * right
}

pub fn div(left: u64, right: u64) -> u64 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = plus(2, 2);
        assert_eq!(result, 4);
    }
}
