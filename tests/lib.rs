mod tests {

    #[test]
    fn english_greeting_hello_correct() {
      assert_eq!("hello", hello_rust::odds_and_ends::greetings::english::hello());
    }

    #[test]
    fn english_greeting_goodbye_correct() {
      assert_eq!("good bye", hello_rust::odds_and_ends::greetings::english::goodbye());
    }
}
