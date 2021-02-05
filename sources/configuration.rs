

use crate::prelude::*;




pub struct Configuration {
	pub endpoint : Endpoint,
	pub handler : Option<HandlerDynArc>,
}


#[ derive (Clone) ]
pub struct Endpoint {
	pub address : EndpointAddress,
	pub protocol : EndpointProtocol,
	pub security : EndpointSecurity,
}


#[ derive (Clone) ]
#[ allow (variant_size_differences) ]
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
	NativeTls (NativeTlsCertificate),
}


#[ derive (Clone) ]
pub struct RustTlsCertificate {
	pub certified : rustls::sign::CertifiedKey,
}

#[ derive (Clone) ]
pub struct NativeTlsCertificate {
	pub identity : natls::Identity,
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
		
		let _security = if false {
			let _certificate = RustTlsCertificate::localhost () .or_panic (0xf64b30c4);
			EndpointSecurity::RustTls (_certificate)
		} else {
			let _certificate = NativeTlsCertificate::localhost () .or_panic (0xf6a595a9);
			EndpointSecurity::NativeTls (_certificate)
		};
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8443)));
		_endpoint.security = _security;
		
		_endpoint
	}
}




pub struct ConfigurationBuilder {
	endpoint : Option<Endpoint>,
	handler : Option<HandlerDynArc>,
	routes : Option<RoutesBuilder>,
}


impl ConfigurationBuilder {
	
	pub fn new () -> Self {
		Self {
				endpoint : None,
				handler : None,
				routes : None,
			}
	}
	
	pub fn build (self) -> ServerResult<Configuration> {
		
		let ConfigurationBuilder {
				endpoint : _endpoint,
				handler : _handler,
				routes : _routes,
			} = self;
		
		let _endpoint = if let Some (_endpoint) = _endpoint {
			_endpoint
		} else {
			Endpoint::default ()
		};
		
		if _handler.is_some () && _routes.is_some () {
			return Err (error_with_message (0xc7d24cd3, "both handler and routes specified"))
		}
		
		let _handler = if let Some (_handler) = _handler {
			Some (_handler)
		} else if let Some (_routes) = _routes {
			let _handler = _routes.build () ? .into_boxed ();
			Some (_handler)
		} else {
			None
		};
		
		let _configuration = Configuration {
				endpoint : _endpoint,
				handler : _handler,
			};
		
		Ok (_configuration)
	}
}


impl ConfigurationBuilder {
	
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
	
	fn endpoint_mut (&mut self) -> &mut Endpoint {
		self.endpoint.get_or_insert_with (Endpoint::default)
	}
}


impl ConfigurationBuilder {
	
	pub fn with_endpoint_certificate_rustls (mut self, _certificate : RustTlsCertificate) -> Self {
		self.endpoint_mut () .security = EndpointSecurity::RustTls (_certificate);
		self
	}
	
	pub fn with_endpoint_certificate_rustls_from_pem_file (self, _path : impl AsRef<path::Path>) -> ServerResult<Self> {
		let _certificate = RustTlsCertificate::load_from_pem_file (_path) ?;
		Ok (self.with_endpoint_certificate_rustls (_certificate))
	}
	
	pub fn with_endpoint_certificate_rustls_from_pem_data (self, _data : impl AsRef<[u8]>) -> ServerResult<Self> {
		let _certificate = RustTlsCertificate::load_from_pem_data (_data) ?;
		Ok (self.with_endpoint_certificate_rustls (_certificate))
	}
}


impl ConfigurationBuilder {
	
	pub fn with_endpoint_certificate_native (mut self, _certificate : NativeTlsCertificate) -> Self {
		self.endpoint_mut () .security = EndpointSecurity::NativeTls (_certificate);
		self
	}
	
	pub fn with_endpoint_certificate_native_from_pkcs12_file (self, _path : impl AsRef<path::Path>, _password : &str) -> ServerResult<Self> {
		let _certificate = NativeTlsCertificate::load_from_pkcs12_file (_path, _password) ?;
		Ok (self.with_endpoint_certificate_native (_certificate))
	}
	
	pub fn with_endpoint_certificate_native_from_pkcs12_data (self, _data : impl AsRef<[u8]>, _password : &str) -> ServerResult<Self> {
		let _certificate = NativeTlsCertificate::load_from_pkcs12_data (_data, _password) ?;
		Ok (self.with_endpoint_certificate_native (_certificate))
	}
}


impl ConfigurationBuilder {
	
	pub fn with_handler <I, H, F, RB, RBE> (self, _handler : I) -> Self
			where
				I : Into<H>,
				H : Handler<Future = F, ResponseBody = RB, ResponseBodyError = RBE> + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : H = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_fn_sync <H, C, RB, RBE> (self, _handler : H) -> Self
			where
				H : Into<HandlerFnSync<C, RB, RBE>>,
				C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnSync<C, RB, RBE> = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_fn_async <H, C, F, RB, RBE> (self, _handler : H) -> Self
			where
				H : Into<HandlerFnAsync<C, F, RB, RBE>>,
				C : Fn (Request<Body>) -> F + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnAsync<C, F, RB, RBE> = _handler.into ();
		self.with_handler_dyn (_handler.into_boxed ())
	}
	
	pub fn with_handler_dyn (mut self, _handler : HandlerDynArc) -> Self
	{
		self.handler = Some (_handler);
		self
	}
}


impl ConfigurationBuilder {
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route <'a, P, I, H, F, RB, RBE> (self, _paths : P, _handler : I) -> Self
			where
				P : Into<RoutePaths<'a>>,
				I : Into<H>,
				H : Handler<Future = F, ResponseBody = RB, ResponseBodyError = RBE> + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : H = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_sync <'a, P, H, C, RB, RBE> (self, _paths : P, _handler : H) -> Self
			where
				P : Into<RoutePaths<'a>>,
				H : Into<HandlerFnSync<C, RB, RBE>>,
				C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnSync<C, RB, RBE> = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_async <'a, P, H, C, F, RB, RBE> (self, _paths : P, _handler : H) -> Self
			where
				P : Into<RoutePaths<'a>>,
				H : Into<HandlerFnAsync<C, F, RB, RBE>>,
				C : Fn (Request<Body>) -> F + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnAsync<C, F, RB, RBE> = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_dyn <'a, P> (mut self, _paths : P, _handler : HandlerDynArc) -> Self
			where
				P : Into<RoutePaths<'a>>,
	{
		let _routes = self.routes.take () .unwrap_or_else (RoutesBuilder::new);
		let _routes = _routes.with_route_dyn (_paths, _handler);
		self.routes = Some (_routes);
		self
	}
}




impl RustTlsCertificate {
	
	pub fn load_from_pem_file (_path : impl AsRef<path::Path>) -> ServerResult<Self> {
		let _data = fs::read (_path) ?;
		Self::load_from_pem_data (&_data)
	}
	
	pub fn load_from_pem_data (_data : impl AsRef<[u8]>) -> ServerResult<Self> {
		
		let _data = _data.as_ref ();
		
		let _certificates = {
			let mut _data = _data;
			rustls_pem::certs (&mut _data) .or_wrap (0x1004be65) ?
		};
		let _private_keys = {
			let mut _data = _data;
			rustls_pem::pkcs8_private_keys (&mut _data) .or_wrap (0x57b13036) ?
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
		Self::load_from_pem_data (_bundle)
	}
}




impl NativeTlsCertificate {
	
	pub fn load_from_pkcs12_file (_path : impl AsRef<path::Path>, _password : &str) -> ServerResult<Self> {
		let _data = fs::read (_path) ?;
		Self::load_from_pkcs12_data (&_data, _password)
	}
	
	pub fn load_from_pkcs12_data (_data : impl AsRef<[u8]>, _password : &str) -> ServerResult<Self> {
		
		let _data = _data.as_ref ();
		
		let _identity = natls::Identity::from_pkcs12 (_data, _password) .or_wrap (0x93817715) ?;
		
		let _certificate = NativeTlsCertificate {
				identity : _identity,
			};
		
		Ok (_certificate)
	}
	
	pub fn localhost () -> ServerResult<Self> {
		let _bundle = include_bytes! ("../examples/tls/testing--server--rsa--bundle.p12");
		Self::load_from_pkcs12_data (_bundle, "bundle")
	}
}

