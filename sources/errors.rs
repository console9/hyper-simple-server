

use crate::prelude::*;




pub type ServerError = io::Error;
pub type ServerResult<Value = ()> = Result<Value, ServerError>;




pub(crate) trait ResultExtPanic<Value, Error : error::Error> : Sized {
	
	fn result (self) -> Result<Value, Error>;
	
	fn or_panic (self, _code : u32) -> Value {
		match self.result () {
			Ok (_value) =>
				_value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
}


impl <Value, Error : error::Error> ResultExtPanic<Value, Error> for Result<Value, Error> {
	
	fn result (self) -> Self {
		self
	}
}


impl <Value> ResultExtPanic<Value, io::Error> for Result<Value, ()> {
	
	fn result (self) -> Result<Value, io::Error> {
		self.map_err (|_| io::Error::new (io::ErrorKind::Other, "unspecified error"))
	}
}




pub(crate) trait ErrorExtPanic<Error : error::Error> : Sized {
	
	fn error (self) -> Error;
	
	fn panic (self, _code : u32) -> ! {
		panic! ("[{:08x}]  unexpected error encountered!  //  {}", _code, self.error ());
	}
}


impl <Error : error::Error> ErrorExtPanic<Error> for Error {
	
	fn error (self) -> Self {
		self
	}
}




pub(crate) trait ResultExtWrap<Value> : Sized {
	
	fn or_wrap (self, _code : u32) -> ServerResult<Value>;
}


impl <Value, Error : error::Error> ResultExtWrap<Value> for Result<Value, Error> {
	
	fn or_wrap (self, _code : u32) -> ServerResult<Value> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (_error.wrap (_code)),
		}
	}
}


pub(crate) trait ErrorExtWrap : Sized {
	
	fn wrap (self, _code : u32) -> ServerError;
}

impl <Error : error::Error> ErrorExtWrap for Error {
	
	fn wrap (self, _code : u32) -> ServerError {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, self))
	}
}




#[ allow (dead_code) ]
pub(crate) fn error_with_format (_code : u32, _message : fmt::Arguments) -> ServerError {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
}

#[ allow (dead_code) ]
pub(crate) fn error_with_message (_code : u32, _message : &str) -> ServerError {
	if ! _message.is_empty () {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
	} else {
		error_with_code (_code)
	}
}

#[ allow (dead_code) ]
pub(crate) fn error_with_code (_code : u32) -> ServerError {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
}




#[ allow (dead_code) ]
pub(crate) fn panic_with_format (_code : u32, _message : fmt::Arguments) -> ! {
	panic! (format! ("[{:08x}]  {}", _code, _message))
}

#[ allow (dead_code) ]
pub(crate) fn panic_with_message (_code : u32, _message : &str) -> ! {
	if ! _message.is_empty () {
		panic! (format! ("[{:08x}]  {}", _code, _message))
	} else {
		panic_with_code (_code)
	}
}

#[ allow (dead_code) ]
pub(crate) fn panic_with_code (_code : u32) -> ! {
	panic! (format! ("[{:08x}]  unexpected error encountered!", _code))
}

