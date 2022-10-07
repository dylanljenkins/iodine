#[cfg(test)]
mod tests {
    struct Input {
        number: usize,
    }

    fn test() {
        println!("{}", 1)
    }

    #[test]
    fn _dummy() {
        assert_eq!(1, 1);
    }

    iodine::generate_tests!("filename", test, Input);
}
