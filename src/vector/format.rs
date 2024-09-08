use super::{Field, Vector};
use std::fmt::{self, Debug, Display};

impl<K: Display + Field, const SIZE: usize> fmt::Display for Vector<K, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for contained in self {
            write!(f, "{}, ", contained)?;
        }
        write!(f, "]\n")
    }
}

impl<K: Debug, const SIZE: usize> fmt::Debug for Vector<K, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")?;
        for contained in self {
            write!(f, "{:?}, ", contained)?;
        }
        write!(f, "\n")
    }
}
