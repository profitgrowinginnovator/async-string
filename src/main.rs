

use bindings::exports::sasync::guest::string_if::{Guest, GuestStringRs};
use wit_bindgen::generate;


mod bindings {
    wit_bindgen::generate!({
        path: "./wit/async-string.wit",
        world: "string-guest",
        async: true,
    });

    use super::AsyncString;
    export!(AsyncString);
}

pub struct AsyncString;


// ✅ Implement `GuestStringRs` for `AsyncString`

impl GuestStringRs for AsyncString {


    async fn string_fn(&self) -> String {
        "success".to_string()
    }
    
    
}

impl Guest for AsyncString {
    type StringRs = AsyncString;
}


fn main() {

    println!("WASM executed successfully!");
}