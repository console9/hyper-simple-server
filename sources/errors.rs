

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
				panic! ("[{:08x}]  unexpected error encountered!  //  {}", _code, _error),
		}
	}
}


impl <Value, Error : error::Error> ResultExtPanic<Value, Error> for Result<Value, Error> {
	
	fn result (self) -> Self {
		self
	}
}




pub(crate) trait ResultExtWrap<Value> : Sized {
	
	fn or_wrap (self) -> ServerResult<Value>;
}


impl <Value, Error : error::Error + Send + Sync + 'static> ResultExtWrap<Value> for Result<Value, Error> {
	
	fn or_wrap (self) -> ServerResult<Value> {
		match self {
			Ok (_value) =>
				Ok (_value),
			Err (_error) =>
				Err (io::Error::new (io::ErrorKind::Other, Box::new (_error))),
		}
	}
}




pub fn error_with_format (_code : u32, _message : fmt::Arguments) -> ServerError {
	io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
}

pub fn error_with_message (_code : u32, _message : &str) -> ServerError {
	if ! _message.is_empty () {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  {}", _code, _message))
	} else {
		io::Error::new (io::ErrorKind::Other, format! ("[{:08x}]  unexpected error encountered!", _code))
	}
}

