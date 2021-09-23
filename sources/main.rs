

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler (_handler : impl Handler, _configuration : Option<Configuration>) -> ServerResult {
	
	let _configuration = prepare_configuration (_configuration) ?;
	
	run_with_handler (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler (_handler : impl Handler, mut _configuration : Configuration) -> ServerResult {
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn, _configuration : Option<Configuration>) -> ServerResult {
	
	let _configuration = prepare_configuration (_configuration) ?;
	
	run_with_handler_dyn (_handler, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_handler_dyn (_handler : impl HandlerDyn, mut _configuration : Configuration) -> ServerResult {
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-core") ]
pub fn main_with_routes (_routes : impl Into<Routes>, _configuration : Option<Configuration>) -> ServerResult {
	
	let _configuration = prepare_configuration (_configuration) ?;
	
	run_with_routes (_routes, _configuration)
}

#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-core") ]
pub fn run_with_routes (_routes : impl Into<Routes>, mut _configuration : Configuration) -> ServerResult {
	
	_configuration.handler = Some (HandlerDynArc::new (_routes.into ()));
	
	Server::run_and_wait (_configuration)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn prepare_configuration_http () -> ServerResult<Configuration> {
	
	let _configuration = Configuration::localhost_http () .build () ?;
	
	prepare_configuration (Some (_configuration))
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-tls-any") ]
pub fn prepare_configuration_https () -> ServerResult<Configuration> {
	
	let _configuration = Configuration::localhost_https () .build () ?;
	
	main_configuration (Some (_configuration))
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
pub fn prepare_configuration (_configuration : Option<Configuration>) -> ServerResult<Configuration> {
	
	let _configuration = if let Some (_configuration) = _configuration {
		_configuration
	} else {
		Configuration::localhost_http () .build () ?
	};
	
	#[ cfg (feature = "hss-cli") ]
	let _configuration = ConfigurationArguments::parse (_configuration) ?;
	
	Ok (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-cli") ]
pub fn prepare_configuration_with_extensions (_configuration : Option<Configuration>, _extensions : impl CliExtensions) -> ServerResult<Configuration> {
	
	let _configuration = if let Some (_configuration) = _configuration {
		_configuration
	} else {
		Configuration::localhost_http () .build () ?
	};
	
	let _configuration = ConfigurationArguments::parse_with_extensions (_configuration, _extensions) ?;
	
	Ok (_configuration)
}

