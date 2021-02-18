

use crate::prelude::*;




pub(crate) mod exports {
	
	pub use super::ServerError;
	pub use super::ServerResult;
}


#[ cfg (feature = "hss-internals") ]
pub(crate) mod internals {
	
	pub use super::ServerError;
	pub use super::ServerResult;
	
	pub use super::ResultExtPanic;
	pub use super::ErrorExtPanic;
	
	pub use super::ResultExtWrap;
	pub use super::ErrorExtWrap;
	
	pub use super::error_with_format;
	pub use super::error_with_message;
	pub use super::error_with_code;
	
	pub use super::panic_with_format;
	pub use super::panic_with_message;
	pub use super::panic_with_code;
}




pub type ServerError = io::Error;
pub type ServerResult<V = ()> = Result<V, ServerError>;




pub trait ResultExtPanic<V, E : Error> : Sized {
	
	fn result (self) -> Result<V, E>;
	
	fn or_panic (self, _code : u32) -> V {
		match self.result () {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
}


impl <V, E : Error> ResultExtPanic<V, E> for Result<V, E> {
	
	fn result (self) -> Self {
		self
	}
}


impl <V> ResultExtPanic<V, io::Error> for Result<V, ()> {
	
	fn result (self) -> Result<V, io::Error> {
		self.map_err (|_| io::Error::new (io::ErrorKind::Other, "unspecified error"))
	}
}




pub trait ErrorExtPanic<E : Error> : Sized {
	
	fn error (self) -> E;
	
	fn panic (self, _code : u32) -> ! {
		panic! ("[{:08x}]  unexpected error encountered!  //  {}", _code, self.error ());
	}
}


impl <E : Error> ErrorExtPanic<E> for E {
	
	fn error (self) -> Self {
		self
	}
}




pub trait ResultExtWrap<V> : Sized {
	
	fn or_wrap (self, _code : u32) -> ServerResult<V>;
}


impl <V, E : Error> ResultExtWrap<V> for Result<V, E> {
	
	fn or_wrap (self, _code : u32) -> ServerResult<V> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.wrap (_code)),
		}
	}
}




pub trait ErrorExtWrap : Sized {
	
	fn wrap (self, _code : u32) -> ServerError;
}

impl <E : Error> ErrorExtWrap for E {
	
	fn wrap (self, _code : u32) -> ServerError {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, self))
	}
}




#[ allow (dead_code) ]
pub fn error_with_format (_code : u32, _message : fmt::Arguments<'_>) -> ServerError {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
}

#[ allow (dead_code) ]
pub fn error_with_message (_code : u32, _message : &str) -> ServerError {
	if ! _message.is_empty () {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
	} else {
		error_with_code (_code)
	}
}

#[ allow (dead_code) ]
pub fn error_with_code (_code : u32) -> ServerError {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
}




#[ allow (dead_code) ]
pub fn panic_with_format (_code : u32, _message : fmt::Arguments<'_>) -> ! {
	panic! (format! ("[{:08x}]  {}", _code, _message))
}

#[ allow (dead_code) ]
pub fn panic_with_message (_code : u32, _message : &str) -> ! {
	if ! _message.is_empty () {
		panic! (format! ("[{:08x}]  {}", _code, _message))
	} else {
		panic_with_code (_code)
	}
}

#[ allow (dead_code) ]
pub fn panic_with_code (_code : u32) -> ! {
	panic! (format! ("[{:08x}]  unexpected error encountered!", _code))
}

