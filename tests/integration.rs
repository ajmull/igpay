#[cfg(test)]
mod tests {
    #[test]
    fn test_to_platin() {
        let foo = "Hello, amazing world!";

        assert_eq!("ellohay amazinghay orldway", igpay::to_platin(foo));
    }
}
