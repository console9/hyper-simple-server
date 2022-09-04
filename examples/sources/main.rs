

use ::hyper_simple_server as hss;

use ::std::sync::Arc;

use hss::ResponseExtBuild as _;
use hss::ResultExtWrap as _;




pub fn main () -> hss::MainResult {
	
	let mut _arguments = hss::CliArguments::from_args ();
	
	type Main = fn (hss::CliArguments) -> hss::MainResult;
	
	let (_main, _arguments_remove_first) : (Main, _) = match _arguments.first_str () {
		
		None =>
			(main_with_hello, false),
		Some ("hello") =>
			(main_with_hello, true),
		Some ("echo") =>
			(main_with_echo, true),
		Some ("routes") =>
			(main_with_routes, true),
		Some ("handler_sync") =>
			(main_with_handler_sync, true),
		Some (_main) =>
			hss::fail! (0x639fb853, "invalid main `{}`", _main),
	};
	
	if _arguments_remove_first {
		_arguments.remove_first_str ();
	}
	
	return _main (_arguments);
}




pub fn main_with_routes (_arguments : hss::CliArguments) -> hss::MainResult {
	
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
			.build ()
			.else_wrap (0xf35f8732) ?;
	
	return hss::main_with_routes (_routes, None, Some (_arguments));
}




pub fn main_with_handler_sync (_arguments : hss::CliArguments) -> hss::MainResult {
	
	eprintln! ("[ii] [e3d58d00]  starting `handler_sync` server...");
	
	let _handler = |_request : hss::Request<hss::Body>| {
		hss::Response::new_200 () .ok ()
	};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	let _handler = Arc::new (_handler);
	
	return hss::main_with_handler (_handler, None, Some (_arguments));
}




pub fn main_with_echo (_arguments : hss::CliArguments) -> hss::MainResult {
	
	eprintln! ("[ii] [53dc40d3]  starting `echo` server...");
	
	let _handler = |_request : hss::Request<hss::Body>| {
		let _uri = _request.uri ();
		let _output = format! ("`{}` | scheme: {:?} | authority: {:?} | path: {:?} | query: {:?}", _uri, _uri.scheme (), _uri.authority (), _uri.path (), _uri.query ());
		hss::Response::new_200_with_text (_output) .ok ()
	};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	let _handler = Arc::new (_handler);
	
	return hss::main_with_handler (_handler, None, Some (_arguments));
}




pub fn main_with_hello (_arguments : hss::CliArguments) -> hss::MainResult {
	
	eprintln! ("[ii] [8759f202]  starting `hello` server...");
	
	let _handler = |_request : hss::Request<hss::Body>| {
		hss::Response::new_200_with_text ("hello world!\n") .ok ()
	};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	let _handler = Arc::new (_handler);
	
	return hss::main_with_handler (_handler, None, Some (_arguments));
}

