mod charref;
mod error;
mod inputstream;
mod token;
mod tokenizer;

#[cfg(test)]
mod html5libtests;

use match_cfg::match_cfg;
use std::fmt;

pub use crate::error::Error;
pub use crate::error::ErrorCode;
pub use crate::token::Attrs;
pub use crate::token::Comment;
pub use crate::token::Doctype;
pub use crate::token::Tag;
pub use crate::token::TagKind;
pub use crate::token::Text;
pub use crate::token::Token;
pub use crate::tokenizer::InitialState;
pub use crate::tokenizer::Tokenizer;

match_cfg! {
    #[cfg(test)] => {
        use serde::Deserialize;

        #[derive(Clone, Copy, Debug, PartialEq)]
        #[derive(Deserialize)]
        pub struct Location {
            pub line: usize,
            pub column: usize,
        }

        impl Location {
            pub fn incr(&mut self) {
                self.column += 1;
            }

            pub fn incr_line(&mut self) {
                self.line += 1;
                self.column = 1;
            }

            pub fn offset(&self, offset: i32) -> Location {
                Location {
                    line: self.line,
                    column: if offset < 0 {
                        self.column - (-offset) as usize
                    } else {
                        self.column + offset as usize
                    },
                }
            }
        }

        impl Default for Location {
            fn default() -> Self {
                Location {
                    line: 1,
                    column: 1,
                }
            }
        }

        impl fmt::Display for Location {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Line#{} Column#{}", self.line, self.column)
            }
        }
    }
    _ => {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Location;

        impl Location {
            pub fn incr(&mut self) {}
            pub fn incr_line(&mut self) {}
            pub fn offset(&self, _: i32) -> Location { Location }
        }

        impl fmt::Display for Location {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "(No location data)")
            }
        }
    }
}
