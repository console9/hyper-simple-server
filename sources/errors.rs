

#![ allow (dead_code) ]




use crate::prelude::*;




pub(crate) mod exports {
	
	pub use ::vrl_errors::{
			StdIoError,
			StdIoResult,
		};
	
	pub use super::{
			
			ServerError,
			ServerResult,
			
		};
}




pub type ServerError = StdIoError;
pub type ServerResult<V = ()> = StdIoResult<V>;


