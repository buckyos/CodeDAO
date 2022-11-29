mod action;
mod cache;
mod config;
mod db;
mod git;
mod global;
mod handler_util;
mod model;
mod object_type;
mod path;
mod post_context;
mod response;
mod stack_util;
mod stat;

pub use action::*;
pub use cache::*;
pub use config::*;
pub use db::*;
pub use git::*;
pub use global::*;
pub use handler_util::*;
pub use model::*;
pub use object_type::*;
pub use path::*;
pub use post_context::*;
pub use response::*;
pub use stack_util::*;
pub use stat::*;

use std::error::Error;
use std::fmt::{self, Debug, Display};
pub struct CodedaoError {
    msg: String,
}
pub type CodedaoResult<T> = Result<T, CodedaoError>;

impl Error for CodedaoError {}
impl Display for CodedaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.msg, f)
    }
}

impl Debug for CodedaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.msg, f)
    }
}

impl From<cyfs_base::BuckyError> for CodedaoError {
    fn from(err: cyfs_base::BuckyError) -> CodedaoError {
        CodedaoError {
            msg: err.to_string(),
        }
    }
}

impl From<git2::Error> for CodedaoError {
    fn from(err: git2::Error) -> CodedaoError {
        CodedaoError {
            msg: err.to_string(),
        }
    }
}
