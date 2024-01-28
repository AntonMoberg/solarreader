use crate::meta::MetaError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Cover(Option<String>);

impl Cover {
    pub fn new<T: Into<Option<String>>>(cover: T) -> Self {
        let cover: Option<String> = cover.into();

        match cover {
            Some(cover) => {
                if !cover.trim().is_empty() {
                    Self(Some(cover))
                } else {
                    Self(None)
                }
            }
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

