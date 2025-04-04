mod cat;
mod echo;
mod exit;
mod ls;
mod mkdir;
mod rm;
mod touch;

use alloc::vec::Vec;
pub use cat::cat;
pub use echo::echo;
pub use exit::exit;
pub use ls::ls;
pub use mkdir::mkdir;
pub use rm::rm;
pub use touch::touch;

use super::{
    parse::Args,
    utils::{ArgParseError, ArgParser},
};

pub fn validate_checked<'a, const N: usize>(
    args: Args,
    named_args: Vec<&str>,
) -> Result<ArgParser<'a, N>, ArgParseError> {
    match ArgParser::try_from(args) {
        Ok(v) => match v.validate(named_args) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
