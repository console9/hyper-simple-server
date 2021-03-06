

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
		let _self = self.internals.read () .or_wrap (0x0f9770a1) ?;
		Self::handler_0 (&_self.configuration)
	}
	
	fn handler_0 (_configuration : &Configuration) -> ServerResult<HandlerDynArc> {
		if let Some (_handler) = _configuration.handler.clone () {
			Ok (_handler)
		} else {
			Err (error_with_message (0x55a5104c, "no handler specified"))
		}
	}
}


#[ cfg (feature = "hss-server-http") ]
#[ cfg (feature = "hss-handler") ]
impl Server
{
	pub fn run_and_wait_with_handler <H, F> (_configuration : Configuration, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _server = Server::new (_configuration) ?;
		_server.serve_and_wait_with_handler (_handler)
	}
	
	pub async fn run_with_handler <H, F> (_configuration : Configuration, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _server = Server::new (_configuration) ?;
		_server.serve_with_handler (_handler) .await
	}
	
	pub fn serve_and_wait_with_handler <H, F> (&self, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _runtime = self.serve_runtime () ?;
		return _runtime.block_on (self.serve_with_handler (_handler));
	}
	
	pub async fn serve_with_handler <H, F> (&self, _handler : H) -> ServerResult
			where
				H : Handler<Future = F> + Send + Sync + 'static + Clone,
				F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
	{
		let _service = move |_ : &Connection| {
				let _handler = _handler.clone () .wrap ();
				async {
					ServerResult::Ok (_handler)
				}
			};
		
		self.serve_with_make_service_fn (_service) .await
	}
}


#[ cfg (feature = "hss-server-http") ]
impl Server {
	
	pub fn serve_builder (&self) -> ServerResult<hyper::Builder<Accepter, ServerExecutor>> {
		
		let _self = self.internals.read () .or_panic (0x1d2cfbb8);
		
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
		
		let _self = self.internals.read () .or_panic (0x6db68b39);
		
		maybe_start_jemalloc_stats ();
		maybe_start_strace ();
		
		let mut _builder_0 = None;
		
		#[ cfg (feature = "hss-server-mt") ]
		if let Some (_threads) = _self.configuration.threads {
			if _threads > 0 {
				#[ cfg (debug_assertions) ]
				eprintln! ("[ii] [cf4d96e6]  starting tokio multi-threaded executor...");
				let mut _builder = tokio::RuntimeBuilder::new_multi_thread ();
				_builder.worker_threads (_threads);
				_builder.max_blocking_threads (_threads * 4);
				_builder.thread_keep_alive (time::Duration::from_secs (60));
				_builder_0 = Some (_builder);
			}
		}
		
		if _builder_0.is_none () {
			#[ cfg (debug_assertions) ]
			eprintln! ("[ii] [25065ee8]  starting tokio current-thread executor...");
			let _builder = tokio::RuntimeBuilder::new_current_thread ();
			_builder_0 = Some (_builder);
		};
		
		let mut _builder = _builder_0.infallible (0xfb2d7cfb);
		
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




#[ cfg (feature = "hss-server-http") ]
fn maybe_start_strace () -> () {
	#[ cfg (feature = "hss-server-debug-strace") ]
	{
		process::Command::new ("strace")
				.args (&["-f", "-p", & process::id () .to_string ()])
				.spawn ()
				.or_panic (0xff87ffef);
	}
}

#[ cfg (feature = "hss-server-http") ]
fn maybe_start_jemalloc_stats () -> () {
	#[ cfg (feature = "hss-server-debug-jemalloc") ]
	{
		extern "C" fn _write (_ : * mut os::raw::c_void, _message : * const os::raw::c_char) {
			#[ allow (unsafe_code) ]
			let _message = unsafe { ffi::CStr::from_ptr (_message) };
			let _message = _message.to_str () .or_panic (0x2d88d281);
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
}

