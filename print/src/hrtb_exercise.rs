trait DisplayMessage {
    // fn display_message(&self) -> &str;
    fn display_message<'a>(&self) -> &'a str;
}

// Implement DisplayMessage for String and &'static str
// TODO: Your implementations here

impl DisplayMessage for String {
    fn display_message<'a>(&self) -> &'a str {
    // fn display_message(&self) -> &str {
        self.as_str()
    }
}

impl DisplayMessage for &'static str {
    fn display_message<'a>(&self) -> &'a str {
        self
    }
}

fn main() {
    let callback = |msg: &dyn DisplayMessage| { println!("LOG: {}", msg.display_message()); };
    log_message(callback);
}

// This is where you'll use HRTBs
// TODO: Specify that F is a function that accepts any type implementing DisplayMessage for any lifetime
fn log_message<T, F>(callback: F) where T: for<'a> DisplayMessage, F: Fn(T), {
    // let message = "This is a log message.".to_string(); // Mock message, you might want to replace or extend this
    let message = "This is a log message."; // Mock message, you might want to replace or extend this
    callback(message);
    // TODO: Incorporate system time and complete the logging logic
}
