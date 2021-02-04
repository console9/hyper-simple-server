

use crate::prelude::*;




pub fn main () -> () {
	
	let _handler_0 = |_request| {
		Ok (Response::new (Body::from ("OK-0")))
	};
	let _handler_1 = |_request| {
		Ok (Response::new (Body::from ("OK-1")))
	};
	let _handler_2 = |_request| {
		Ok (Response::new (Body::from ("OK-2")))
	};
	
	let _configuration = Configuration::localhost_https ()
			.with_route_fn_sync ("", _handler_0)
			.with_route_fn_sync (&["/1", "/1/"], _handler_1)
			.with_route_fn_sync (&["/2", "/2/*any"], _handler_2)
			.build () .or_panic (0xb601cf12);
	
	Server::run_and_wait (_configuration) .or_panic (0x267c0521);
}

