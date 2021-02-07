

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler (_handler : impl Handler) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_handler (_handler)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler_dyn (_handler : impl HandlerDyn) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_handler_dyn (_handler)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_routes (_routes : impl Into<Routes>) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_routes (_routes)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}

