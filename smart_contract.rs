#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang::contract;

contract! {
    // Declare your storage variables
    struct MyContract {
        my_number: i32,
        my_string: String,
    }

    impl MyContract {
        // Constructor function to initialize storage variables
        #[ink(constructor)]
        fn new(&mut self, initial_number: i32, initial_string: String) {
            self.my_number = initial_number;
            self.my_string = initial_string;
        }

        // Function to get the current value of the my_number variable
        #[ink(message)]
        fn get_my_number(&self) -> i32 {
            self.my_number
        }

        // Function to set the value of the my_number variable
        #[ink(message)]
        fn set_my_number(&mut self, new_number: i32) {
            self.my_number = new_number;
        }

        // Function to get the current value of the my_string variable
        #[ink(message)]
        fn get_my_string(&self) -> String {
            self.my_string.clone()
        }

        // Function to set the value of the my_string variable
        #[ink(message)]
        fn set_my_string(&mut self, new_string: String) {
            self.my_string = new_string;
        }
    }
}
