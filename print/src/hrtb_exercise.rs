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

fn log_message<T, F>(callback: F, message: &T)
where
  T: for<'a> DisplayMessage + ?Sized,
  F: for<'a> Fn(&'a T),
{
    let message = "This is a log message."; // Mock message, you might want to replace or extend this
    callback(&message as &dyn DisplayMessage);
}
