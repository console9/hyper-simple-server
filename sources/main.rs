

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler <H : Handler> (_handler : H) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_handler (_handler)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_handler_dyn <H : HandlerDyn> (_handler : H) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_handler_dyn (_handler)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}


#[ cfg (feature = "hss-main") ]
#[ cfg (feature = "hss-routes") ]
#[ cfg (feature = "hss-server-http") ]
pub fn main_with_routes <I : Into<Routes>> (_routes : I) -> ServerResult {
	
	let _configuration = Configuration::localhost_http ()
			.with_routes (_routes)
			.build () ?;
	
	Server::run_and_wait (_configuration)
}

