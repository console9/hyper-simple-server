

use crate::prelude::*;




#[ derive (Clone) ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
pub struct Server {
	internals : ServerInternals,
}

#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
struct ServerInternals0 {
	configuration : Configuration,
}

#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
type ServerInternals = Arc<RwLock<ServerInternals0>>;




#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
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


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl Server {
	
	pub fn run_and_wait (_configuration : Configuration) -> ServerResult {
		let _handler = Self::handler_0 (&_configuration) ?;
		Self::run_and_wait_with_handler (_configuration, _handler)
	}
	
	pub async fn run (_configuration : Configuration) -> ServerResult {
		let _handler = Self::handler_0 (&_configuration) ?;
		Self::run_with_handler (_configuration, _handler) .await
	}
	
	pub fn serve_and_wait (&self) -> ServerResult {
		let _handler = self.handler () ?;
		self.serve_and_wait_with_handler (_handler)
	}
	
	pub async fn serve (&self) -> ServerResult {
		let _handler = self.handler () ?;
		self.serve_with_handler (_handler) .await
	}
	
	fn handler (&self) -> ServerResult<HandlerDynArc> {
		let _self = self.internals.read () .unwrap ();  // FIXME:  infallible
		Self::handler_0 (&_self.configuration)
	}
	
	fn handler_0 (_configuration : &Configuration) -> ServerResult<HandlerDynArc> {
		if let Some (_handler) = _configuration.handler.clone () {
			Ok (_handler)
		} else {
			fail! (0x55a5104c, "no handler specified");
		}
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl Server
{
	pub fn run_and_wait_with_handler <H, F> (_configuration : Configuration, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = StdIoResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _server = Server::new (_configuration) ?;
		_server.serve_and_wait_with_handler (_handler)
	}
	
	pub async fn run_with_handler <H, F> (_configuration : Configuration, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = StdIoResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _server = Server::new (_configuration) ?;
		_server.serve_with_handler (_handler) .await
	}
	
	pub fn serve_and_wait_with_handler <H, F> (&self, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = StdIoResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		#[ cfg (feature = "hss-server-profiling") ]
		let _profiling = {
			let _self = self.internals.read ();  // FIXME:  infallible
			if let Some (_path) = &_self.configuration.profiling {
				Some (ProfilingSession::new_and_start (_path) ?)
			} else {
				None
			}
		};
		
		let _runtime = self.serve_runtime () ?;
		let _future = self.serve_with_handler (_handler);
		let _outcome = _runtime.block_on (_future);
		
		#[ cfg (feature = "hss-server-profiling") ]
		if let Some (_profiling) = _profiling {
			_profiling.stop_and_drop () ?;
		}
		
		_outcome
	}
	
	pub async fn serve_with_handler <H, F> (&self, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = StdIoResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _service = move |_ : &Connection| {
				let _service = _handler.clone () .wrap ();
				let _service = ServiceWrapper (_service);
				async {
					ServerResult::Ok (_service)
				}
			};
		
		self.serve_with_make_service_fn (_service) .await
	}
}




#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl Server {
	
	pub fn serve_builder (&self) -> ServerResult<hyper::Builder<Accepter, ServerExecutor>> {
		
		let _self = self.internals.read () .unwrap ();  // FIXME:  infallible
		
		eprintln! ("[ii] [83af6f05]  server listening on `{}`;", _self.configuration.endpoint.url ());
		
		let _accepter = Accepter::new (&_self.configuration.endpoint) .else_wrap (0x2579fd0d) ?;
		let _protocol = self.serve_protocol () ?;
		let _executor = ServerExecutor ();
		
		let _builder = hyper::Builder::new (_accepter, _protocol);
		let _builder = _builder.executor (_executor);
		
		Ok (_builder)
	}
	
	pub async fn serve_with_service_fn <S, SF, SB, SBD> (&self, _service : S) -> ServerResult
			where
				S : FnMut (Request<Body>) -> SF + Send + 'static + Clone,
				SF : Future<Output = StdIoResult<Response<SB>>> + Send + 'static,
				SB : BodyTrait<Data = SBD, Error = StdIoError> + Send + Sync + 'static,
				SBD : Buf + Send + 'static,
	{
		let _make_service = move |_ : &Connection| {
				let _service = _service.clone ();
				let _service = hyper::service_fn (_service);
				let _service = ServiceWrapper (_service);
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
				SB : BodyTrait<Data = SBD, Error = SBE> + Send + Sync + 'static,
				SBD : Buf + Send + 'static,
				SBE : Error + Send + Sync + 'static,
	{
		
		let _service = hyper::make_service_fn (_make_service);
		let _builder = self.serve_builder () ?;
		
		let _future = _builder.serve (_service);
		let _future = _future.with_graceful_shutdown (async { tokio::ctrl_c () .await .else_panic (0xa011830e); });
		
		#[ cfg (debug_assertions) ]
		eprintln! ("[ii] [3aed0938]  server initialized;");
		
		let _outcome = _future.await;
		
		#[ cfg (debug_assertions) ]
		eprintln! ("[ii] [3eff9778]  server terminated;");
		
		let _outcome = _outcome.else_wrap (0x73080376);
		_outcome
	}
	
	pub fn serve_protocol (&self) -> ServerResult<hyper::Http> {
		
		let _self = self.internals.read () .unwrap ();  // FIXME:  infallible
		let _protocol = &_self.configuration.endpoint.protocol;
		
		let mut _http = hyper::Http::new ();
		
		#[ cfg (feature = "hyper--server-http1") ]
		if _protocol.supports_http1_only () {
			_http.http1_only (true);
		}
		#[ cfg (feature = "hyper--server-http1") ]
		if _protocol.supports_http1 () {
			_http.http1_keep_alive (true);
			_http.http1_half_close (true);
			_http.max_buf_size (16 * 1024);
		}
		
		#[ cfg (feature = "hyper--server-http2") ]
		if _protocol.supports_http2_only () {
			_http.http2_only (true);
		}
		#[ cfg (feature = "hyper--server-http2") ]
		if _protocol.supports_http2 () {
			_http.http2_max_concurrent_streams (128);
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_interval (Some (time::Duration::new (6, 0)));
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_timeout (time::Duration::new (30, 0));
		}
		
		Ok (_http)
	}
	
	pub fn serve_runtime (&self) -> ServerResult<Runtime> {
		
		let _self = self.internals.read () .unwrap ();  // FIXME:  infallible
		
		#[ cfg (feature = "hss-jemalloc") ]
		if true {
			#[ cfg (debug_assertions) ]
			eprintln! ("[ii] [cecdcf1b]  using `jemalloc` allocator;");
			#[ cfg (feature = "hss-server-debug-jemalloc") ]
			server_start_jemalloc_stats ();
		}
		
		#[ cfg (feature = "hss-server-debug-strace") ]
		if true {
			server_start_strace ();
		}
		
		let mut _runtime_0 = None;
		
		#[ cfg (feature = "hss-server-mt") ]
		if let Some (_threads) = _self.configuration.threads {
			if _threads > 0 {
				#[ cfg (debug_assertions) ]
				eprintln! ("[ii] [cf4d96e6]  using multi-threaded executor (with {} threads);", _threads);
				let _runtime = runtime_multiple_threads (Some (_threads)) ?;
				_runtime_0 = Some (_runtime);
			}
		}
		
		if _runtime_0.is_none () {
			#[ cfg (debug_assertions) ]
			eprintln! ("[ii] [25065ee8]  using current-thread executor (with 1 thread);");
			let _runtime = runtime_current_thread () ?;
			_runtime_0 = Some (_runtime);
		};
		
		let _runtime = _runtime_0.infallible (0xfb2d7cfb);
		
		#[ cfg (feature = "hss-server-sanitize") ]
		#[ cfg (debug_assertions) ]
		eprintln! ("[ii] [3c1badd4]  using URI sanitizer;");
		
		Ok (_runtime)
	}
}




#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
struct ServiceWrapper <S> (S)
	where
		S : hyper::Service<Request<Body>, Error = StdIoError>,
;

#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
#[ allow (dead_code) ]
enum ServiceWrapperFuture <S>
	where
		S : hyper::Service<Request<Body>, Error = StdIoError>,
{
	Future (S::Future),
	Error (StdIoError),
	Done,
}


#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl <S> hyper::Service<Request<Body>> for ServiceWrapper<S>
	where
		S : hyper::Service<Request<Body>, Error = StdIoError>,
{
	type Future = ServiceWrapperFuture<S>;
	type Response = S::Response;
	type Error = StdIoError;
	
	fn poll_ready (&mut self, _context : &mut Context<'_>) -> Poll<StdIoResult> {
		self.0.poll_ready (_context)
	}
	
	fn call (&mut self, mut _request : Request<Body>) -> Self::Future {
		
		#[ cfg (feature = "hss-server-sanitize") ]
		match sanitize_uri (_request.uri ()) {
			Err (_error) => {
				if true {
					eprintln! ("[ww] [aace2099]  URI sanitize failed for `{}`:  {}", _request.uri (), _error);
				}
				return ServiceWrapperFuture::Error (_error.into_std_io_error ());
			}
			Ok (Some (_uri)) => {
				if true {
					eprintln! ("[ww] [d1e356bc]  URI sanitized to `{}` from `{}`;", _uri, _request.uri ());
				}
				* _request.uri_mut () = _uri;
			}
			Ok (None) => (),
		}
		
		let _future = self.0.call (_request);
		ServiceWrapperFuture::Future (_future)
	}
}

#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl <S> Future for ServiceWrapperFuture<S>
	where
		S : hyper::Service<Request<Body>, Error = StdIoError>,
{
	type Output = <S::Future as Future>::Output;
	
	fn poll (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Self::Output> {
		#[ allow (unsafe_code) ]
		let _self_0 = unsafe { self.get_unchecked_mut () };
		match _self_0 {
			ServiceWrapperFuture::Future (_future) => {
				#[ allow (unsafe_code) ]
				let _delegate = unsafe { Pin::new_unchecked (_future) };
				match _delegate.poll (_context) {
					_outcome @ Poll::Pending =>
						_outcome,
					_outcome @ Poll::Ready (Ok (_)) => {
						let _ = mem::replace (_self_0, ServiceWrapperFuture::Done);
						_outcome
					}
					Poll::Ready (Err (_error)) => {
						if true {
							eprintln! ("[ee] [540dc2bc]  handler failed:  {}", _error);
						}
						Poll::Ready (Err (_error))
					}
				}
			}
			ServiceWrapperFuture::Error (_error) => {
				let _self_1 = mem::replace (_self_0, ServiceWrapperFuture::Done);
				if let ServiceWrapperFuture::Error (_error) = _self_1 {
					Poll::Ready (Err (_error))
				} else {
					panic! (enforcement, 0xd83566d8);
				}
			}
			ServiceWrapperFuture::Done =>
				Poll::Ready (Err (failed! (ServerError, 0x0722e578) .into_std_io_error ())),  // FIXME:  ???
		}
	}
}




#[ derive (Clone) ]
#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
pub struct ServerExecutor ();

#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hyper--server") ]
impl <F> hyper::Executor<F> for ServerExecutor
		where
			F : Future<Output = ()> + Send + 'static,
{
	fn execute (&self, _future : F) {
		tokio::spawn (_future);
	}
}




#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-server-debug-strace") ]
fn server_start_strace () -> () {
	
	#[ cfg (debug_assertions) ]
	eprintln! ("[ii] [19f96abc]  starting `strace` tracing...");
	
	process::Command::new ("strace")
			.args (&["-f", "-p", & process::id () .to_string ()])
			.spawn ()
			.else_panic (0xff87ffef);
}


#[ cfg (feature = "hss-server-core") ]
#[ cfg (feature = "hss-jemalloc") ]
#[ cfg (feature = "hss-server-debug-jemalloc") ]
fn server_start_jemalloc_stats () -> () {
	
	#[ cfg (debug_assertions) ]
	eprintln! ("[ii] [19f5dcf1]  starting `jemalloc` tracing...");
	
	extern "C" fn _write (_ : * mut os::raw::c_void, _message : * const os::raw::c_char) {
		#[ allow (unsafe_code) ]
		let _message = unsafe { ffi::CStr::from_ptr (_message) };
		let _message = _message.to_str () .infallible (0x2d88d281);
		for _message in _message.split_terminator ("\n") {
			if (_message == "___ Begin jemalloc statistics ___") || (_message == "--- End jemalloc statistics ---") {
				continue;
			}
			if _message == "Background threads: 0, num_runs: 0, run_interval: 0 ns" {
				continue;
			}
			eprintln! ("[dd] [35256205]  jemalloc statistics:  {}", _message);
		}
	}
	thread::spawn (|| {
		let _options = &b"gmdablxe\0"[..];
			loop {
				#[ allow (unsafe_code) ]
				unsafe { ::jemalloc_sys::malloc_stats_print (Some (_write), ptr::null_mut (), _options.as_ptr () as * const os::raw::c_char) };
				thread::sleep (time::Duration::from_secs (1));
		}
	});
}




#[ cfg (feature = "tokio--rt-multi-thread") ]
pub fn runtime_multiple_threads (_threads : Option<usize>) -> ServerResult<Runtime> {
	let _threads = _threads.unwrap_or (1);
	let mut _builder = tokio::RuntimeBuilder::new_multi_thread ();
	_builder.worker_threads (_threads);
	_builder.max_blocking_threads (_threads * 4);
	_builder.thread_keep_alive (time::Duration::from_secs (60));
	_builder.enable_all ();
	_builder.build () .else_wrap (0x2692223a)
}

#[ cfg (feature = "tokio--rt") ]
pub fn runtime_current_thread () -> ServerResult<Runtime> {
	let mut _builder = tokio::RuntimeBuilder::new_current_thread ();
	_builder.enable_all ();
	_builder.build () .else_wrap (0x280fcb72)
}

