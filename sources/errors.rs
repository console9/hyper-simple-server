

#![ allow (dead_code) ]




use crate::prelude::*;




pub(crate) mod exports {
	
	pub use ::vrl_errors::{
			StdIoError,
			StdIoResult,
		};
	
	pub use super::{
			*,
		};
}




define_error! (pub HandlerError, result : HandlerResult);
define_error! (pub ServerError, result : ServerResult);
define_error! (pub ConfigurationError, result : ConfigurationResult);
define_error! (pub RoutesError, result : RoutesResult);
define_error! (pub SanitizeError, result : SanitizeResult);
define_error! (pub CliError, result : CliResult);
define_error! (pub MainError, result : MainResult);
define_error! (pub AccepterError, result : AccepterResult);
define_error! (pub ConnectionError, result : ConnectionResult);
define_error! (pub ProfilingError, result : ProfilingResult);


