

use ::hyper_simple_server as hss;

use hss::ResponseExtBuild as _;




fn main () -> hss::ServerResult {
	
	if false {
		return main_with_static ();
	} else {
		return main_with_routes ();
	}
}




fn main_with_routes () -> hss::ServerResult {
	
	let _handler_0 = |_request| {
		hss::Response::new_200 () .ok ()
	};
	let _handler_1 = |_request| {
		hss::Response::new_200_with_text ("OK-1") .ok ()
	};
	let _handler_2 = |_request| {
		hss::Response::new_200_with_text ("OK-2") .ok ()
	};
	let _handler_x = |_request| {
		hss::Response::new_404 () .ok ()
	};
	
	let _routes = hss::Routes::builder ()
			.with_route_fn_sync ("", _handler_x)
			.with_route_fn_sync ("/", _handler_0)
			.with_route_fn_sync (&["/1", "/1/"], _handler_1)
			.with_route_fn_sync (&["/2", "/2/*any"], _handler_2)
			.build () ?;
	
	return hss::main_with_routes (_routes, None);
}




fn main_with_static () -> hss::ServerResult {
	
	let _handler = |_request : hss::Request<hss::Body>| {
		hss::Response::new_200 () .ok_0 ()
	};
	let _handler = hss::HandlerFnSync::from (_handler);
	
	return hss::main_with_handler (_handler, None);
}

