#[cfg(test)]
mod tests {
    #[derive(serde::Deserialize)]
    struct Input {
        number: usize,
    }

    fn test(input: Input) {
        println!("{}", input.number)
    }

    #[test]
    #[ignore]
    /// Dummy test so that CLion sees this test file as runnable.
    fn _dummy() {
        assert_eq!(1, 1);
    }

    iodine::generate_tests!("tests/test_data.json", test, Input);
}
