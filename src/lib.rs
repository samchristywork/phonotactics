pub fn foo() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        foo();
    }
}
