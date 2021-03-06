use std::borrow::Cow;
use std::fmt;

use rand::{self, Rng};
use rocket::request::FromParam;

const BASE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {
    pub fn new(size: usize) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            id.push(BASE[rng.gen::<usize>() % 62] as char)
        }

        PasteId(Cow::Owned(id))
    }
}

impl fmt::Display for PasteId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param.chars().all(|c| c.is_ascii_alphanumeric()) {
            true => Ok(PasteId(param.into())),
            false => Err(param),
        }
    }
}
