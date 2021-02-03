

use crate::prelude::*;




pub struct Configuration {
	pub endpoint : Endpoint,
	pub handler : HandlerDynArc,
}




#[ derive (Clone) ]
pub struct Endpoint {
	pub address : EndpointAddress,
	pub protocol : EndpointProtocol,
	pub security : EndpointSecurity,
}


#[ derive (Clone) ]
pub enum EndpointAddress {
	Socket (net::SocketAddr),
	Descriptor (u32),
}


#[ derive (Clone) ]
pub enum EndpointProtocol {
	Http1,
	Http2,
	Http12,
}


#[ derive (Clone) ]
pub enum EndpointSecurity {
	Insecure,
	RustTls (Arc<rustls::ServerConfig>),
}




impl Configuration {
	
	pub fn builder () -> ConfigurationBuilder {
		ConfigurationBuilder::new ()
	}
}


impl Default for Endpoint {
	
	fn default () -> Self {
		Endpoint {
				address : EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 0))),
				protocol : EndpointProtocol::Http1,
				security : EndpointSecurity::Insecure,
			}
	}
}


impl Endpoint {
	
	pub fn example_http () -> Self {
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8080)));
		
		_endpoint
	}
	
	pub fn example_https () -> Self {
		
		let _certificates = {
			let mut _certificate_data = & include_bytes! ("../examples/self-signed--certificate.pem") [..];
			let _certificates = rustls::internal::pemfile::certs (&mut _certificate_data) .or_panic (0x6ed75325);
			if _certificates.is_empty () {
				panic_with_message (0xc6991697, "no certificates loaded");
			}
			_certificates
		};
		
		let _private_key = {
			let mut _private_key_data = & include_bytes! ("../examples/self-signed--private-key.pem") [..];
			let _private_keys = rustls::internal::pemfile::pkcs8_private_keys (&mut _private_key_data) .or_panic (0x71cd79a6);
			if _private_keys.len () == 1 {
				_private_keys.into_iter () .next () .unwrap ()
			} else if _private_keys.is_empty () {
				panic_with_message (0x84af61dd, "no private key loaded");
			} else {
				panic_with_message (0xa5a124ef, "multiple private keys loaded");
			}
		};
		
		let mut _tls = rustls::ServerConfig::new (rustls::NoClientAuth::new ());
		_tls.set_single_cert (_certificates, _private_key) .expect ("[77a6398d]");
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8443)));
		_endpoint.security = EndpointSecurity::RustTls (Arc::new (_tls));
		
		_endpoint
	}
}




#[ derive (Default) ]
pub struct ConfigurationBuilder {
	endpoint : Option<Endpoint>,
	handler : Option<HandlerDynArc>,
}


impl ConfigurationBuilder {
	
	pub fn new () -> Self {
		Self { .. Default::default () }
	}
	
	pub fn build (self) -> ServerResult<Configuration> {
		
		let ConfigurationBuilder {
				endpoint : _endpoint,
				handler : _handler,
			} = self;
		
		let _endpoint = if let Some (_endpoint) = _endpoint {
			_endpoint
		} else {
			Endpoint::default ()
		};
		
		let _handler = if let Some (_handler) = _handler {
			_handler
		} else {
			return Err (error_with_message (0x83e7297f, "missing handler"));
		};
		
		let _configuration = Configuration {
				endpoint : _endpoint,
				handler : _handler,
			};
		
		Ok (_configuration)
	}
	
	pub fn with_endpoint (mut self, _endpoint : Endpoint) -> Self {
		self.endpoint = Some (_endpoint);
		self
	}
	
	pub fn with_handler <I, H, F> (self, _handler : I) -> Self
			where
				I : Into<H>,
				H : Handler<Future = F> + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response>> + Send + 'static
	{
		let _handler : H = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_fn_sync <H, C> (self, _fn : H) -> Self
			where
				H : Into<HandlerFnSync<C>>,
				C : Fn (Request) -> ServerResult<Response> + Send + Sync + 'static
	{
		let _handler : HandlerFnSync<C> = _fn.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_fn_async <H, C, F> (self, _fn : H) -> Self
			where
				H : Into<HandlerFnAsync<C, F>>,
				C : Fn (Request) -> F + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response>> + Send + 'static
	{
		let _handler : HandlerFnAsync<C, F> = _fn.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_dyn (mut self, _handler : HandlerDynArc) -> Self
	{
		self.handler = Some (_handler);
		self
	}
}

