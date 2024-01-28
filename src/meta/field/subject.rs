use crate::meta::MetaError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Subject(Option<String>);

impl Subject {
    pub fn new<T: Into<Option<String>>>(subject: T) -> Self {
        let subject: Option<String> = subject.into();

        match subject {
            Some(subject) => {
                if !subject.trim().is_empty() {
                    Self(Some(subject))
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

impl FromStr for Subject {
    type Err = MetaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
