

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
			
			error_with_format,
			error_with_message,
			error_with_code,
			error_wrap,
			
			panic_with_format,
			panic_with_message,
			panic_with_code,
			
		};
}




pub type ServerError = StdIoError;
pub type ServerResult<V = ()> = StdIoResult<V>;




pub fn error_with_format (_code : u32, _message : fmt::Arguments<'_>) -> StdIoError {
	StdIoError::new (StdIoErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
}

pub fn error_with_message (_code : u32, _message : &str) -> StdIoError {
	if ! _message.is_empty () {
		StdIoError::new (StdIoErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
	} else {
		error_with_code (_code)
	}
}

pub fn error_with_code (_code : u32) -> StdIoError {
	StdIoError::new (StdIoErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
}

pub fn error_wrap <E : Error> (_code : u32, _error : E) -> StdIoError {
	StdIoError::new (StdIoErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!  //  {}", _code, _error))
}




pub fn panic_with_format (_code : u32, _message : fmt::Arguments<'_>) -> ! {
	::std::panic! ("[{:08x}]  {}", _code, _message)
}

pub fn panic_with_message (_code : u32, _message : &str) -> ! {
	if ! _message.is_empty () {
		::std::panic! ("[{:08x}]  {}", _code, _message)
	} else {
		panic_with_code (_code)
	}
}

pub fn panic_with_code (_code : u32) -> ! {
	::std::panic! ("[{:08x}]  unexpected error encountered!", _code)
}

