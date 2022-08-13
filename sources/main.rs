

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler (_handler : impl Handler, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult {
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_handler (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler (_handler : impl Handler, mut _configuration : Configuration) -> MainResult {
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration) .else_wrap (0xa6450541)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn, _configuration : Option<Configuration>, _arguments : Option<CliArguments>) -> MainResult {
	
	let _configuration = prepare_configuration (_configuration, _arguments) ?;
	
	run_with_handler_dyn (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler_dyn (_handler : impl HandlerDyn, mut _configuration : Configuration) -> MainResult {
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration) .else_wrap (0x0a736397)
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
	
	_configuration.handler = Some (HandlerDynArc::new (_routes.into ()));
	
	Server::run_and_wait (_configuration) .else_wrap (0x614ed7fa)
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
	
	#[ cfg (feature = "hss-cli") ]
	let _configuration = ConfigurationArguments::parse (_configuration, _arguments) .else_wrap (0x51bf7a0e) ?;
	
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

