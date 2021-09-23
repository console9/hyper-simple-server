

use ::hyper_simple_server as hss;

use hss::ResponseExtBuild as _;




fn main () -> hss::ServerResult {
	
	let _main = "echo";
	
	let _main = match std::env::var ("HSS_EXAMPLE_MAIN") {
		Ok (_value) => _value,
		Err (std::env::VarError::NotPresent) => String::from (_main),
		Err (_error) => Err (hss::error_wrap (0xfe7f8e93, _error)) ?,
	};
	
	let _main = match _main.as_str () {
		
		"echo" => main_with_echo,
		"routes" => main_with_routes,
		"handler_sync" => main_with_handler_sync,
		
		_ => Err (hss::error_with_format (0xa5dbefb7, format_args! ("invalid main `{}`", _main))) ?,
	};
	
	return _main ();
}




fn main_with_routes () -> hss::ServerResult {
	
	eprintln! ("[ii] [f8f30521]  starting `routes` server...");
	
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




fn main_with_handler_sync () -> hss::ServerResult {
	
	eprintln! ("[ii] [e3d58d00]  starting `handler_sync` server...");
	
	let _handler = |_request : hss::Request<hss::Body>| {
		hss::Response::new_200 () .ok_0 ()
	};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	
	return hss::main_with_handler (_handler, None);
}




fn main_with_echo () -> hss::ServerResult {
	
	eprintln! ("[ii] [53dc40d3]  starting `echo` server...");
	
	let _handler = |_request : hss::Request<hss::Body>| {
		let _uri = _request.uri ();
		let _output = format! ("`{}` | scheme: {:?} | authority: {:?} | path: {:?} | query: {:?}", _uri, _uri.scheme (), _uri.authority (), _uri.path (), _uri.query ());
		hss::Response::new_200_with_text (_output) .ok_0 ()
	};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	
	return hss::main_with_handler (_handler, None);
}

