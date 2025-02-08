use serde::Deserialize;
// conver Json into a Rust struct

#[derive(Deserialize)]
// Deserialization means converting data from another format(like Json) into Rust struct

// It saves time and reduces bugs! Instead of writing complex conversion code, you just add #[derive(Deserialize)], and serde does all the hard work for you.
pub struct User {
    pub name: String,
    pub email: String,
}
