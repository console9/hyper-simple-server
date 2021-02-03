

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
		
		let _bundle_data = & include_bytes! ("../examples/tls/testing--server--rsa--bundle.pem") [..];
		
		let _certificate_chain = {
			let mut _certificate_data = _bundle_data;
			let _certificates = rustls_pem::certs (&mut _certificate_data) .or_panic (0x1d1d6f0f);
			if _certificates.is_empty () {
				panic_with_message (0xc6991697, "no certificates loaded");
			}
			_certificates.into_iter () .map (rustls::Certificate) .collect ()
		};
		
		let _private_key = {
			let mut _private_key_data = _bundle_data;
			let _private_keys = rustls_pem::pkcs8_private_keys (&mut _private_key_data) .or_panic (0x71cd79a6);
			let mut _private_keys = _private_keys.into_iter ();
			if let Some (_private_key) = _private_keys.next () {
				if _private_keys.next () .is_some () {
					panic_with_message (0xa5a124ef, "multiple private keys loaded");
				}
				rustls::PrivateKey (_private_key)
			} else {
				panic_with_message (0x84af61dd, "no private key loaded");
			}
		};
		
		let _certified = {
			let _private_key = rustls::sign::any_supported_type (&_private_key) .or_panic (0x1a5e250d);
			rustls::sign::CertifiedKey::new (_certificate_chain, Arc::new (_private_key))
		};
		
		let _resolver = {
			struct Resolver (rustls::sign::CertifiedKey);
			impl rustls::ResolvesServerCert for Resolver {
				fn resolve (&self, _: rustls::ClientHello) -> Option<rustls::sign::CertifiedKey> {
					Some (self.0.clone ())
				}
			}
			Resolver (_certified)
		};
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		let _tls = {
			let mut _tls = rustls::ServerConfig::new (rustls::NoClientAuth::new ());
			_tls.cert_resolver = Arc::new (_resolver);
			match _endpoint.protocol {
				EndpointProtocol::Http1 =>
					_tls.alpn_protocols.push ("http/1.1".into ()),
				EndpointProtocol::Http2 =>
					_tls.alpn_protocols.push ("h2".into ()),
				EndpointProtocol::Http12 => {
					_tls.alpn_protocols.push ("h2".into ());
					_tls.alpn_protocols.push ("http/1.1".into ());
				}
			}
			_tls
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

