

use crate::prelude::*;


pub struct Server {
	internals : ServerInternals,
}

struct ServerInternals0 {
	configuration : Configuration,
}

type ServerInternals = Arc<RwLock<ServerInternals0>>;




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
	
	pub fn serve_and_wait (&self) -> ServerResult {
		let mut _runtime = tokio::Runtime::new () .or_panic (0x8b2d6703);
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


impl Server {
	
	pub fn run_and_wait (_configuration : Configuration) -> ServerResult {
		let _server = Server::new (_configuration) ?;
		_server.serve_and_wait ()
	}
	
	pub async fn run (_configuration : Configuration) -> ServerResult {
		let _server = Server::new (_configuration) ?;
		_server.serve () .await
	}
}


impl Server {
	
	pub fn serve_builder (&self) -> ServerResult<hyper::Builder<Accepter, ServerExecutor>> {
		
		let _self = self.internals.read () .or_panic (0x6db68b39);
		
		let _accepter = Accepter::new (&_self.configuration.endpoint) ?;
		let _protocol = _accepter.protocol () .deref () .clone ();
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
}




#[ derive (Clone) ]
pub struct ServerExecutor ();

impl <F> hyper::Executor<F> for ServerExecutor
		where
			F : Future<Output = ()> + Send + 'static,
{
	fn execute (&self, _future : F) {
		tokio::spawn (_future);
	}
}

