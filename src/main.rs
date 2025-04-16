mod domain;
mod tests;


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod unittests {
    #[test]
    fn test_hello_world() {
        assert!(true); // Placeholder test
    }
}
