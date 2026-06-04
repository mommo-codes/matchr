pub fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lowercase() {
		assert_eq!(normalize("Hello"), "hello");
	}

	#[test]
	fn test_trim() {
		assert_eq!(normalize(" hello "), "hello");
	}

	#[test]
	fn test_both() {
		assert_eq!(normalize(" Hello World "), "hello world")
	}
}


