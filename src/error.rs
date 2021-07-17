use core::fmt::{Display, Formatter};
use fmt;

/// Describes the error that can happen when the "multiboot2 information structure" (*mbi*)
/// gets loaded.
#[derive(Debug)]
pub enum MbiLoadError {
    IllegalAddress,
    IllegalTotalSize(u32),
    NoEndTag,
}

impl Display for MbiLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MbiLoadError::IllegalAddress => {
                write!(f, "The address is illegal and doesn't point to MBI.")
            }
            MbiLoadError::IllegalTotalSize(s) => {
                write!(f, "The size of the MBI is illegal: {:x}", s)
            }
            MbiLoadError::NoEndTag => {
                write!(f, "the MBI doesn't contain an \"end\"-tag.")
            }
        }
    }
}

