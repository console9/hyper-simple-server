

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler (_handler : impl Handler + Clone, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult {
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_handler (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler (_handler : impl Handler + Clone, mut _configuration : Configuration) -> MainResult {
	
	Server::run_and_wait_with_handler (_configuration, _handler) .else_wrap (0xa6450541)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn + Clone, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult {
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_handler_dyn (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler_dyn (_handler : impl HandlerDyn + Clone, mut _configuration : Configuration) -> MainResult {
	
	let _handler = HandlerDynArc::new (_handler);
	
	Server::run_and_wait_with_handler (_configuration, _handler) .else_wrap (0x0a736397)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_routes (_routes : impl Into<Routes>, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult {
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_routes (_routes, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_routes (_routes : impl Into<Routes>, mut _configuration : Configuration) -> MainResult {
	
	let _routes = _routes.into ();
	
	Server::run_and_wait_with_handler (_configuration, _routes) .else_wrap (0x614ed7fa)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_service_fn <S, SF, SB, SBD, SBE> (_service : S, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult
			where
				S : FnMut (Request<Body>) -> SF + Send + 'static + Clone,
				SF : Future<Output = ServiceResult<Response<SB>>> + Send + 'static,
				SB : BodyTrait<Data = SBD, Error = SBE> + Send + Sync + 'static,
				SBD : Buf + Send + 'static,
				SBE : StdError + Send + Sync + 'static,
{
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_service_fn (_service, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_service_fn <S, SF, SB, SBD, SBE> (_service : S, mut _configuration : Configuration) -> MainResult
		where
			S : FnMut (Request<Body>) -> SF + Send + 'static + Clone,
			SF : Future<Output = ServiceResult<Response<SB>>> + Send + 'static,
			SB : BodyTrait<Data = SBD, Error = SBE> + Send + Sync + 'static,
			SBD : Buf + Send + 'static,
			SBE : StdError + Send + Sync + 'static,
{
	
	Server::run_and_wait_with_service_fn (_configuration, _service) .else_wrap (0x3248947e)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_make_service_fn <M, MF, ME, S, SF, SE, SB, SBD, SBE> (_make_service : M, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult
		where
			M : FnMut (&Connection) -> MF + Send + 'static,
			MF : Future<Output = Result<S, ME>> + Send + 'static,
			ME : StdError + Send + Sync + 'static,
			S : hyper::Service<Request<Body>, Response = Response<SB>, Future = SF, Error = SE> + Send + 'static,
			SE : StdError + Send + Sync + 'static,
			SF : Future<Output = Result<Response<SB>, SE>> + Send + 'static,
			SB : BodyTrait<Data = SBD, Error = SBE> + Send + Sync + 'static,
			SBD : Buf + Send + 'static,
			SBE : StdError + Send + Sync + 'static,
{
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_make_service_fn (_make_service, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_make_service_fn <M, MF, ME, S, SF, SE, SB, SBD, SBE> (_make_service : M, mut _configuration : Configuration) -> MainResult
		where
			M : FnMut (&Connection) -> MF + Send + 'static,
			MF : Future<Output = Result<S, ME>> + Send + 'static,
			ME : StdError + Send + Sync + 'static,
			S : hyper::Service<Request<Body>, Response = Response<SB>, Future = SF, Error = SE> + Send + 'static,
			SE : StdError + Send + Sync + 'static,
			SF : Future<Output = Result<Response<SB>, SE>> + Send + 'static,
			SB : BodyTrait<Data = SBD, Error = SBE> + Send + Sync + 'static,
			SBD : Buf + Send + 'static,
			SBE : StdError + Send + Sync + 'static,
{
	
	Server::run_and_wait_with_make_service_fn (_configuration, _make_service) .else_wrap (0xb83ede3b)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn prepare_configuration_http (_arguments : Option<CliArguments>) -> MainResult<Configuration> {
	
	let _configuration = Configuration::localhost_http () .build () .else_wrap (0x39773c7b) ?;
	
	prepare_configuration (Some (_configuration), _arguments)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-tls-any") ]
pub fn prepare_configuration_https (_arguments : Option<CliArguments>) -> MainResult<Configuration> {
	
	let _configuration = Configuration::localhost_https () .build () .else_wrap (0x6cef5637) ?;
	
	prepare_configuration (Some (_configuration), _arguments)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn prepare_configuration (_configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult<Configuration> {
	
	let _configuration = if let Some (_configuration) = _configuration {
		_configuration
	} else {
		Configuration::localhost_http () .build () .else_wrap (0x032f4aa5) ?
	};
	
	let _arguments = CliArguments::unwrap_or_args (_arguments);
	
	#[ cfg (feature = "hss-cli") ]
	let _configuration = ConfigurationArguments::parse (_configuration, Some (_arguments)) .else_wrap (0x51bf7a0e) ?;
	
	Ok (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-cli") ]
pub fn prepare_configuration_with_extensions (_configuration : Option<Configuration>, _extensions : impl CliExtensions, _arguments : Option<CliArguments>) -> MainResult<Configuration> {
	
	let _configuration = if let Some (_configuration) = _configuration {
		_configuration
	} else {
		Configuration::localhost_http () .build () .else_wrap (0x160dd2bf) ?
	};
	
	let _configuration = ConfigurationArguments::parse_with_extensions (_configuration, _extensions, _arguments) .else_wrap (0xbf170303) ?;
	
	Ok (_configuration)
}

