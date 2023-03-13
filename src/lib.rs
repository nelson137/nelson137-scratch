//! Scratch project for testing things.
//!
//! ## Cross-crate doc links test
//!
//! - [Link to docs.rs `bevy::app::App`](https://docs.rs/bevy/*/bevy/app/struct.App.html)
//! - [Link to crate dep `bevy_app::App`](bevy_app::App)
//! - [Link to crate `bevy::app::App` (broken)](bevy::app::App)

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
