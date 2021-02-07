

use crate::prelude::*;




#[ derive (Clone) ]
#[ cfg (feature = "hss-server-http") ]
pub struct Server {
	internals : ServerInternals,
}

#[ cfg (feature = "hss-server-http") ]
struct ServerInternals0 {
	configuration : Configuration,
}

#[ cfg (feature = "hss-server-http") ]
type ServerInternals = Arc<RwLock<ServerInternals0>>;




#[ cfg (feature = "hss-server-http") ]
impl Server {
	
	pub fn new (_configuration : Configuration) -> ServerResult<Self> {
		let _self = ServerInternals0 {
				configuration : _configuration,
			};
		let _self = Server {
				internals : Arc::new (RwLock::new (_self)),
			};
		Ok (_self)
	}
}


#[ cfg (feature = "hss-server-http") ]
#[ cfg (feature = "hss-handler") ]
impl Server {
	
	pub fn run_and_wait (_configuration : Configuration) -> ServerResult {
		let _server = Server::new (_configuration) ?;
		_server.serve_and_wait ()
	}
	
	pub async fn run (_configuration : Configuration) -> ServerResult {
		let _server = Server::new (_configuration) ?;
		_server.serve () .await
	}
	
	pub fn serve_and_wait (&self) -> ServerResult {
		let _runtime = self.serve_runtime () ?;
		return _runtime.block_on (self.serve ());
	}
	
	pub async fn serve (&self) -> ServerResult {
		
		let _handler = if let Some (_handler) = self.handler () {
			_handler
		} else {
			return Err (error_with_message (0x55a5104c, "no handler specified"));
		};
		
		let _service = move |_ : &Connection| {
				let _handler = _handler.clone ();
				async {
					ServerResult::Ok (_handler)
				}
			};
		
		self.serve_with_make_service_fn (_service) .await
	}
	
	fn handler (&self) -> Option<HandlerDynArc> {
		let _self = self.internals.read () .or_panic (0x6db68b39);
		_self.configuration.handler.clone ()
	}
}


#[ cfg (feature = "hss-server-http") ]
impl Server {
	
	pub fn serve_builder (&self) -> ServerResult<hyper::Builder<Accepter, ServerExecutor>> {
		
		let _self = self.internals.read () .or_panic (0x6db68b39);
		
		let _accepter = Accepter::new (&_self.configuration.endpoint) ?;
		let _protocol = self.serve_protocol () ?;
		let _executor = ServerExecutor ();
		
		let _builder = hyper::Builder::new (_accepter, _protocol);
		let _builder = _builder.executor (_executor);
		
		Ok (_builder)
	}
	
	pub async fn serve_with_service_fn <S, SE, SF, SB, SBD, SBE> (&self, _service : S) -> ServerResult
			where
				S : FnMut (Request<Body>) -> SF + Send + 'static + Clone,
				SE : Error + Send + Sync + 'static,
				SF : Future<Output = Result<Response<SB>, SE>> + Send + 'static,
				SB : BodyTrait<Data = SBD, Error = SBE> + Send + 'static,
				SBD : Buf + Send + 'static,
				SBE : Error + Send + Sync + 'static,
	{
		let _make_service = move |_connection : &Connection| {
				let _service = hyper::service_fn (_service.clone ());
				async {
					ServerResult::Ok (_service)
				}
			};
		self.serve_with_make_service_fn (_make_service).await
	}
	
	pub async fn serve_with_make_service_fn <M, MF, ME, S, SF, SE, SB, SBD, SBE> (&self, _make_service : M) -> ServerResult
			where
				M : FnMut (&Connection) -> MF + Send + 'static,
				MF : Future<Output = Result<S, ME>> + Send + 'static,
				ME : Error + Send + Sync + 'static,
				S : hyper::Service<Request<Body>, Response = Response<SB>, Future = SF, Error = SE> + Send + 'static,
				SE : Error + Send + Sync + 'static,
				SF : Future<Output = Result<Response<SB>, SE>> + Send + 'static,
				SB : BodyTrait<Data = SBD, Error = SBE> + Send + 'static,
				SBD : Buf + Send + 'static,
				SBE : Error + Send + Sync + 'static,
	{
		let _service = hyper::make_service_fn (_make_service);
		let _builder = self.serve_builder () ?;
		_builder.serve (_service) .await .or_wrap (0x73080376)
	}
	
	pub fn serve_protocol (&self) -> ServerResult<hyper::Http> {
		
		let _self = self.internals.read () .or_panic (0x6db68b39);
		let _protocol = &_self.configuration.endpoint.protocol;
		
		let mut _http = hyper::Http::new ();
		
		#[ cfg (feature = "hyper--http1") ]
		if _protocol.supports_http1_only () {
			_http.http1_only (true);
		}
		#[ cfg (feature = "hyper--http1") ]
		if _protocol.supports_http1 () {
			_http.http1_keep_alive (true);
			_http.http1_half_close (true);
			_http.max_buf_size (16 * 1024);
		}
		
		#[ cfg (feature = "hyper--http2") ]
		if _protocol.supports_http2_only () {
			_http.http2_only (true);
		}
		#[ cfg (feature = "hyper--http2") ]
		if _protocol.supports_http2 () {
			_http.http2_max_concurrent_streams (128);
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_interval (Some (time::Duration::new (6, 0)));
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_timeout (time::Duration::new (30, 0));
		}
		
		Ok (_http)
	}
	
	pub fn serve_runtime (&self) -> ServerResult<tokio::Runtime> {
		
		#[ cfg (debug_assertions) ]
		if false {
			process::Command::new ("strace")
					.args (&["-f", "-p", & process::id () .to_string ()])
					.spawn () .or_panic (0xff87ffef);
		}
		
		#[ cfg (not (feature = "tokio--rt-multi-thread")) ]
		let mut _builder = {
			#[ cfg (debug_assertions) ]
			eprintln! ("[ii] [25065ee8]  starting tokio current-thread executor...");
			tokio::RuntimeBuilder::new_current_thread ()
		};
		
		#[ cfg (feature = "tokio--rt-multi-thread") ]
		let mut _builder = {
			#[ cfg (debug_assertions) ]
			eprintln! ("[ii] [cf4d96e6]  starting tokio multi-threaded executor...");
			tokio::RuntimeBuilder::new_multi_thread ()
		};
		
		_builder.enable_all ();
		
		let _runtime = _builder.build () .or_wrap (0xc29071d8) ?;
		
		Ok (_runtime)
	}
}




#[ derive (Clone) ]
#[ cfg (feature = "hss-server-http") ]
pub struct ServerExecutor ();

#[ cfg (feature = "hss-server-http") ]
impl <F> hyper::Executor<F> for ServerExecutor
		where
			F : Future<Output = ()> + Send + 'static,
{
	fn execute (&self, _future : F) {
		tokio::spawn (_future);
	}
}

