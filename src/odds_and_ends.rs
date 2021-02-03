#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rand;
use rand::Rng;

pub fn odds_and_ends() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!("{}", b);
}

// coment

/*
 * comment spamming several lines 
 */

pub mod greetings {
    //!  This module contains English phrases.
    //!
    //! # Examples
    //! ```
    //! let username = "John";
    //! let hello = hello_rust::odds_and_ends::greetings::english::hello();
    //! println!("{}, {}!", hello, username);
    //! ```

    pub mod english {
        /// Applies to code that follows it.
        /// In this case, it's our `hello()` function
        pub fn hello() -> String {
            "hello".to_string()
        }
        pub fn goodbye() -> String {
            "good bye".to_string()
        }
    }
    pub mod french {
        pub fn hello() -> String {
            "bonjour".to_string()
        }
        pub fn goodbye() -> String {
            "au revoir".to_string()
        }
    }
}
