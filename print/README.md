# HRTB Exercise: Logging Framework

========================

This exercise will involve a common use case for HRTBs: working with callbacks that take references with any lifetime. We'll create a simple logging framework that logs messages with a timestamp, but with a twist: we want it to accept any message that can be displayed, regardless of its lifetime.

- HRTBs: working with callbacks that take references with any lifetime
- it should accept any message that can be displayed, regardless of its lifetime

### Background

Imagine you're building a part of a logging system where you can register a callback that processes messages to be logged.
These messages could be string literals (with a `'static` lifetime) or strings constructed at runtime (with a shorter lifetime).
Your goal is to write a function that accepts a callback capable of handling messages of any lifetime.

### Task

[x] 1. **Define a Trait for Displayable Messages**: First, define a trait `DisplayMessage` that has a method `display_message` which takes a ***reference to self (&self)*** and returns a ***string slice (`&str`)***.
This trait will be implemented for types that can be converted into a displayable message.
```
trait DisplayMessage {
    fn display_message(&self) -> &str;
}
```

[x] 2. **Implement `DisplayMessage` for `String` and `&'static str`**: Implement the `DisplayMessage` trait for `String` and for `&'static str` so that they simply return the string slice of themselves.

[ ] 3. **Write a Logger Function**: Write a function `log_message` that accepts a ***generic callback***. This callback should be capable of accepting any type that implements `DisplayMessage` for any lifetime. The `log_message` function will use the current system time (feel free to mock or simplify this) and the callback to log the message.

[ ] 4. **Use HRTBs in `log_message`**: The key part of the exercise is to ensure that the callback in `log_message` can accept messages of any lifetime, not just `'static`. This is where HRTBs come into play.

### Exercise Goals

- Fill in the missing parts of the starter code.
- Make sure `log_message` can accept a callback that works with any message, regardless of its lifetime.
- Test your solution with both a string literal and a `String` object created at runtime.

### Hints

- Think about how the trait `DisplayMessage` and its method might need to declare lifetimes.
- Consider how to express in `log_message`'s type bounds that `F` is a callback that can work with a `T` implementing `DisplayMessage` for any lifetime.

This exercise will challenge you to apply HRTBs in a practical context, reinforcing your understanding of how they enable flexible, lifetime-agnostic code.

