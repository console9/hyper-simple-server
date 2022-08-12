

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
			
			ResultExtPanic,
			ErrorExtPanic,
			
			ResultExtWrap,
			ResultExtWrapFrom,
			ErrorExtWrap,
			ErrorExtWrapFrom,
			
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




pub trait ResultExtPanic <V> : Sized {
	
	fn else_panic (self, _code : u32) -> V;
	
	fn infallible (self, _code : u32) -> V;
}


impl <V, EX : ErrorExtPanic> ResultExtPanic<V> for Result<V, EX> {
	
	fn else_panic (self, _code : u32) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
	
	fn infallible (self, _code : u32) -> V {
		match self {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
}


impl <V> ResultExtPanic<V> for Option<V> {
	
	fn else_panic (self, _code : u32) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic_with_code (_code),
		}
	}
	
	fn infallible (self, _code : u32) -> V {
		match self {
			Some (_value) =>
				_value,
			None =>
				panic_with_code (_code),
		}
	}
}




pub trait ErrorExtPanic : Sized {
	
	fn panic (self, _code : u32) -> !;
}


impl <E : Error> ErrorExtPanic for E {
	
	fn panic (self, _code : u32) -> ! {
		::std::panic! ("[{:08x}]  unexpected error encountered!  //  {}", _code, self);
	}
}




pub trait ResultExtWrap <V, E> : Sized {
	
	fn else_wrap (self, _code : u32) -> Result<V, E>;
}


impl <V, E : Error> ResultExtWrap<V, io::Error> for Result<V, E> {
	
	fn else_wrap (self, _code : u32) -> Result<V, io::Error> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (io::Error::wrap_from (_code, _error)),
		}
	}
}


impl <V> ResultExtWrap<V, io::Error> for Option<V> {
	
	fn else_wrap (self, _code : u32) -> Result<V, io::Error> {
		if let Some (_value) = self {
			Ok (_value)
		} else {
			Err (error_with_code (_code))
		}
	}
}




pub trait ResultExtWrapFrom <V, E> : Sized {
	
	fn or_wrap_from (_code : u32, _result : Result<V, E>) -> Self;
}


impl <V, E : Error, EX : ErrorExtWrapFrom<E>> ResultExtWrapFrom<V, E> for Result<V, EX> {
	
	fn or_wrap_from (_code : u32, _result : Result<V, E>) -> Result<V, EX> {
		match _result {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (EX::wrap_from (_code, _error)),
		}
	}
}




pub trait ErrorExtWrapFrom <E> : Sized {
	
	fn wrap_from (_code : u32, _error : E) -> Self;
}


impl <E : Error> ErrorExtWrapFrom<E> for io::Error {
	
	fn wrap_from (_code : u32, _error : E) -> Self {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _error))
	}
}




pub trait ErrorExtWrap <E> : Sized {
	
	fn wrap (self, _code : u32) -> E;
}


impl <EI, EO : ErrorExtWrapFrom<EI>> ErrorExtWrap<EO> for EI {
	
	fn wrap (self, _code : u32) -> EO {
		EO::wrap_from (_code, self)
	}
}




pub fn error_with_format (_code : u32, _message : fmt::Arguments<'_>) -> io::Error {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
}

pub fn error_with_message (_code : u32, _message : &str) -> io::Error {
	if ! _message.is_empty () {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
	} else {
		error_with_code (_code)
	}
}

pub fn error_with_code (_code : u32) -> io::Error {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
}

pub fn error_wrap <E : Error> (_code : u32, _error : E) -> io::Error {
	io::Error::wrap_from (_code, _error)
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




#[ macro_export ]
macro_rules! fail_with_format {
	( $_code : literal, $_format : literal, $( $_argument : tt )* ) => {
		return ::std::result::Result::Err ($crate::error_with_format ($_code, ::std::format_args! ($_format, $( $_argument )* )))
	}
}

#[ macro_export ]
macro_rules! fail_with_message {
	( $_code : literal, $_message : literal ) => {
		return ::std::result::Result::Err ($crate::error_with_message ($_code, $_message))
	};
}

#[ macro_export ]
macro_rules! fail_with_code {
	( $_code : literal ) => {
		return ::std::result::Result::Err ($crate::error_with_code ($_code))
	};
}

#[ macro_export ]
macro_rules! fail_wrap {
	( $_code : literal, $_error : expr ) => {
		return ::std::result::Result::Err ($crate::error_wrap ($_code, $_error))
	};
}

