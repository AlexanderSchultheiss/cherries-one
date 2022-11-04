struct Hello {
    name: String,
    count: i32,
}

impl Hello {
    fn new() -> Self {
        Hello {
            name: String::from("Hello"),
            count: 0,
        }
    }

    fn with_name(name: &str) -> Self {
        Hello {
            name: String::from(name),
            count: 0,
        }
    }
}
