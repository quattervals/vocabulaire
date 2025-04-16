mod domain;
mod tests;
mod driving;


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
