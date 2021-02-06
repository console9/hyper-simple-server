

use crate::prelude::*;




#[ cfg (feature = "hss-main") ]
pub fn main () -> () {
	
	let _handler_0 = |_request| {
		Response::new_200 () .ok ()
	};
	let _handler_1 = |_request| {
		Response::new_200_with_text ("OK-1") .ok ()
	};
	let _handler_2 = |_request| {
		Response::new_200_with_text ("OK-2") .ok ()
	};
	let _handler_x = |_request| {
		Response::new_404 () .ok ()
	};
	
	let _configuration = Configuration::localhost_http ()
			.with_route_fn_sync ("", _handler_x)
			.with_route_fn_sync ("/", _handler_0)
			.with_route_fn_sync (&["/1", "/1/"], _handler_1)
			.with_route_fn_sync (&["/2", "/2/*any"], _handler_2)
			.build () .or_panic (0xb601cf12);
	
	Server::run_and_wait (_configuration) .or_panic (0x267c0521);
}

