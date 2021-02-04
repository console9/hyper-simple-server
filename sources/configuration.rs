

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
	RustTls (RustTlsCertificate),
}


#[ derive (Clone) ]
pub struct RustTlsCertificate {
	pub certified : rustls::sign::CertifiedKey
}




impl Configuration {
	
	pub fn builder () -> ConfigurationBuilder {
		ConfigurationBuilder::new ()
	}
	
	pub fn localhost_http () -> ConfigurationBuilder {
		Configuration::builder ()
			.with_endpoint (Endpoint::localhost_http ())
	}
	
	pub fn localhost_https () -> ConfigurationBuilder {
		Configuration::builder ()
			.with_endpoint (Endpoint::localhost_https ())
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
	
	pub fn localhost_http () -> Self {
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8080)));
		
		_endpoint
	}
	
	pub fn localhost_https () -> Self {
		
		let _certificate = RustTlsCertificate::localhost () .or_panic (0xf64b30c4);
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8443)));
		_endpoint.security = EndpointSecurity::RustTls (_certificate);
		
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
	
	pub fn with_endpoint_address (mut self, _address : EndpointAddress) -> Self {
		self.endpoint_mut () .address = _address;
		self
	}
	
	pub fn with_endpoint_socket_address (mut self, _address : net::SocketAddr) -> Self {
		self.endpoint_mut () .address = EndpointAddress::Socket (_address);
		self
	}
	
	pub fn with_endpoint_protocol (mut self, _protocol : EndpointProtocol) -> Self {
		self.endpoint_mut () .protocol = _protocol;
		self
	}
	
	pub fn with_endpoint_security (mut self, _security : EndpointSecurity) -> Self {
		self.endpoint_mut () .security = _security;
		self
	}
	
	pub fn with_endpoint_certificate (mut self, _certificate : RustTlsCertificate) -> Self {
		self.endpoint_mut () .security = EndpointSecurity::RustTls (_certificate);
		self
	}
	
	fn endpoint_mut (&mut self) -> &mut Endpoint {
		if let Some (_endpoint) = self.endpoint.as_mut () {
			_endpoint
		} else {
			self.endpoint = Some (Endpoint::default);
			self.endpoint_mut ()
		}
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
	
	pub fn with_handler_fn_sync <H, C> (self, _handler : H) -> Self
			where
				H : Into<HandlerFnSync<C>>,
				C : Fn (Request) -> ServerResult<Response> + Send + Sync + 'static
	{
		let _handler : HandlerFnSync<C> = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_fn_async <H, C, F> (self, _handler : H) -> Self
			where
				H : Into<HandlerFnAsync<C, F>>,
				C : Fn (Request) -> F + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response>> + Send + 'static
	{
		let _handler : HandlerFnAsync<C, F> = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_dyn (mut self, _handler : HandlerDynArc) -> Self
	{
		self.handler = Some (_handler);
		self
	}
}




impl RustTlsCertificate {
	
	pub fn load_from_pem_str (mut _data : &str) -> ServerResult<Self> {
		Self::load_from_pem_bytes (_data.as_bytes ())
	}
	
	pub fn load_from_pem_bytes (_data : &[u8]) -> ServerResult<Self> {
		
		let _certificates = {
			let mut _data = _data;
			rustls_pem::certs (&mut _data) .or_wrap () ?
		};
		let _private_keys = {
			let mut _data = _data;
			rustls_pem::pkcs8_private_keys (&mut _data) .or_wrap () ?
		};
		
		Self::load_from_parts (
				_certificates.iter () .map (|_part| _part.as_slice ()),
				_private_keys.iter () .map (|_part| _part.as_slice ()),
			)
	}
	
	pub fn load_from_parts <'a> (mut _certificates : impl Iterator<Item = &'a [u8]>, mut _private_keys : impl Iterator<Item = &'a [u8]>) -> ServerResult<Self> {
		let _certificates = {
			let _certificates : Vec<_> = _certificates.map (<[u8]>::to_vec) .map (rustls::Certificate) .collect ();
			if _certificates.is_empty () {
				return Err (error_with_message (0xc6991697, "no certificates found"));
			}
			_certificates
		};
		let _private_key = {
			if let Some (_private_key) = _private_keys.next () {
				if _private_keys.next () .is_some () {
					return Err (error_with_message (0xa5a124ef, "multiple private keys found"));
				}
				rustls::PrivateKey (_private_key.to_vec ())
			} else {
				return Err (error_with_message (0x84af61dd, "no private key found"));
			}
		};
		Self::load_from_parts_0 (_certificates, _private_key)
	}
	
	fn load_from_parts_0 (_certificates : Vec<rustls::Certificate>, _private_key : rustls::PrivateKey) -> ServerResult<Self> {
		let _certified = {
			let _private_key = rustls::sign::any_supported_type (&_private_key) .map_err (|_| error_with_message (0x5c4797d0, "invalid private key")) ?;
			rustls::sign::CertifiedKey::new (_certificates, Arc::new (_private_key))
		};
		let _certificate = RustTlsCertificate {
				certified : _certified,
			};
		Ok (_certificate)
	}
	
	pub fn localhost () -> ServerResult<Self> {
		let _bundle = include_str! ("../examples/tls/testing--server--rsa--bundle.pem");
		Self::load_from_pem_str (_bundle)
	}
}

