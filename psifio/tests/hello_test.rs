use psifio;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(psifio::hello_test(), "Hello test");
    }
}