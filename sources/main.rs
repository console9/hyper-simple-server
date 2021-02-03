

use crate::prelude::*;




pub fn main () -> () {
	
	let _handler = |_request| {
		Ok (Response::new (Body::from ("OK")))
	};
	
	let _endpoint = Endpoint::example_https ();
	
	let _configuration = Configuration::builder ()
			.with_handler_fn_sync (_handler)
			.with_endpoint (_endpoint)
			.build () .or_panic (0xb601cf12);
	
	Server::run (_configuration) .or_panic (0xe2bb7eb3);
}

