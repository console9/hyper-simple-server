

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler (_handler : impl Handler) -> ServerResult {
	
	let mut _configuration = main_configuration () ?;
	_configuration.handler = HandlerDynArc::new (_handler) .into ();
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn) -> ServerResult {
	
	let mut _configuration = main_configuration () ?;
	_configuration.handler = HandlerDynArc::new (_handler) .into ();
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_routes (_routes : impl Into<Routes>) -> ServerResult {
	
	let mut _configuration = main_configuration () ?;
	_configuration.handler = HandlerDynArc::new (_routes.into ()) .into ();
	
	Server::run_and_wait (_configuration)
}




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_configuration () -> ServerResult<Configuration> {
	
	let mut _configuration = Configuration::localhost_http ()
			.build () ?;
	
	#[ cfg (feature = "hss-cli") ]
	ConfigurationArguments::parse_and_update (&mut _configuration) ?;
	
	Ok (_configuration)
}

