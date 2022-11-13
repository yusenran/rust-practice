/// 与えられた挨拶の文字列に、Worldを付け足して返す
///
/// # Examples
///
/// ```
/// mod hello;
/// use hello::sub_hello;
///
/// let greeting = sub_hello::hello_to_world();
/// assert_eq!("Hello, World2!", greeting);
/// ```
fn greet_to_world(s: &str) -> String {
    s.to_string() + ", World!"
}

pub fn hello_to_world() -> String {
    greet_to_world("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_to_world() {
        assert_eq!(greet_to_world("Hello"), "Hello, World!");
    }
}
