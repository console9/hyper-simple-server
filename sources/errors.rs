

use crate::prelude::*;




#[ cfg (feature = "hss-handler") ]
::vrl_errors::define_error! (pub HandlerError, result : HandlerResult);

#[ cfg (feature = "hss-server-core") ]
::vrl_errors::define_error! (pub ServerError, result : ServerResult);

#[ cfg (feature = "hss-config") ]
::vrl_errors::define_error! (pub ConfigurationError, result : ConfigurationResult);

#[ cfg (feature = "hss-routes") ]
::vrl_errors::define_error! (pub RoutesError, result : RoutesResult);

#[ cfg (feature = "hss-sanitize") ]
::vrl_errors::define_error! (pub SanitizeError, result : SanitizeResult);

#[ cfg (feature = "hss-cli") ]
::vrl_errors::define_error! (pub CliError, result : CliResult);

#[ cfg (feature = "hss-main") ]
::vrl_errors::define_error! (pub MainError, result : MainResult);

#[ cfg (feature = "hss-accepter") ]
::vrl_errors::define_error! (pub AccepterError, result : AccepterResult);

#[ cfg (feature = "hss-accepter") ]
::vrl_errors::define_error! (pub ConnectionError, result : ConnectionResult);

#[ cfg (feature = "cpuprofiler") ]
::vrl_errors::define_error! (pub ProfilingError, result : ProfilingResult);




#[ cfg (not (feature = "hss-exports")) ]
pub(crate) use self::imports::*;

#[ cfg (feature = "hss-exports") ]
pub use self::imports::*;

mod imports {
	
	pub use ::vrl_errors::{
			
			Error,
			ErrorWithDetails,
			ErrorNew,
			ErrorNewWithDetails,
			
			ErrorExtWrap,
			ErrorExtPanic,
			
			ResultExtWrap,
			ResultExtPanic,
			ResultExtUnexpected,
			
			StdError,
			StdIoError,
			StdIoResult,
			
			failed,
			fail,
			panic,
	};
}


