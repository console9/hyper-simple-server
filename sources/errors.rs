

use crate::prelude::*;




pub type ServerError = io::Error;
pub type ServerResult<Value = ()> = Result<Value, ServerError>;




pub trait ResultExtPanic<Value, Error : error::Error> : Sized {
	
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




pub trait ResultExtWrap<Value> : Sized {
	
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

