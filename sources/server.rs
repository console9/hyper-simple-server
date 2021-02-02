

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
	
	pub fn serve (&self) -> ServerResult {
		let mut _runtime = tokio::Runtime::new () .or_panic (0x8b2d6703);
		return _runtime.block_on (self.serve_0 ());
	}
	
	async fn serve_0 (&self) -> ServerResult {
		
		let _self = self.internals.read () .or_panic (0x6db68b39);
		
		let _accepter = Accepter::new (&_self.configuration.endpoint) ?;
		let _protocol = _accepter.protocol ();
		let _server = hyper::Builder::new (_accepter, _protocol);
		let _handler = _self.configuration.handler.clone ();
		
		drop (_self);
		
		let _service = hyper::make_service_fn (
			|_| {
				let _handler = _handler.clone ();
				async {
					ServerResult::Ok (_handler)
				}
			});
		
		_server.serve (_service) .await .or_wrap ()
	}
	
	pub fn run (_configuration : Configuration) -> ServerResult {
		let _server = Server::new (_configuration) ?;
		_server.serve () ?;
		Ok (())
	}
}

