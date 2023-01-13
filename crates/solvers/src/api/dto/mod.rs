mod auction;
mod solution;

use serde::Serialize;

pub use self::{auction::Auction, solution::Solution};

#[derive(Debug, Serialize)]
pub struct Error {
    pub message: &'static str,
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Self {
        Self { message }
    }
}