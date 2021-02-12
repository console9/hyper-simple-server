

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler (_handler : impl Handler, _configuration : Option<Configuration>) -> ServerResult {
	
	let mut _configuration = main_configuration_from_template (_configuration) ?;
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn, _configuration : Option<Configuration>) -> ServerResult {
	
	let mut _configuration = main_configuration_from_template (_configuration) ?;
	
	_configuration.handler = Some (HandlerDynArc::new (_handler));
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_routes (_routes : impl Into<Routes>, _configuration : Option<Configuration>) -> ServerResult {
	
	let mut _configuration = main_configuration_from_template (_configuration) ?;
	
	_configuration.handler = Some (HandlerDynArc::new (_routes.into ()));
	
	Server::run_and_wait (_configuration)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_configuration_http () -> ServerResult<Configuration> {
	
	let _configuration = Configuration::localhost_http () .build () ?;
	
	main_configuration_from_template (Some (_configuration))
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-http") ]
#[ cfg (feature = "hss-tls-andy") ]
pub fn main_configuration_https () -> ServerResult<Configuration> {
	
	let _configuration = Configuration::localhost_https () .build () ?;
	
	main_configuration_from_template (Some (_configuration))
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_configuration_from_template (_configuration : Option<Configuration>) -> ServerResult<Configuration> {
	
	let mut _configuration = if let Some (_configuration) = _configuration {
		_configuration
	} else {
		Configuration::localhost_http () .build () ?
	};
	
	#[ cfg (feature = "hss-cli") ]
	ConfigurationArguments::parse_and_update (&mut _configuration) ?;
	
	Ok (_configuration)
}

