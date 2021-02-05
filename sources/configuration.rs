

use crate::prelude::*;




#[ cfg (feature = "hss-config") ]
pub struct Configuration {
	pub endpoint : Endpoint,
	#[ cfg (feature = "hss-handler") ]
	pub handler : Option<HandlerDynArc>,
}


#[ derive (Clone) ]
#[ cfg (feature = "hss-config") ]
pub struct Endpoint {
	pub address : EndpointAddress,
	#[ cfg (feature = "hyper--http") ]
	pub protocol : EndpointProtocol,
	pub security : EndpointSecurity,
}


#[ derive (Clone) ]
#[ allow (variant_size_differences) ]
#[ cfg (feature = "hss-config") ]
pub enum EndpointAddress {
	Socket (net::SocketAddr),
	Descriptor (u32),
}


#[ derive (Clone) ]
#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hyper--http") ]
pub enum EndpointProtocol {
	#[ cfg (feature = "hyper--http1") ]
	Http1,
	#[ cfg (feature = "hyper--http2") ]
	Http2,
	#[ cfg (feature = "hyper--http1") ]
	#[ cfg (feature = "hyper--http2") ]
	Http12,
}


#[ derive (Clone) ]
#[ cfg (feature = "hss-config") ]
pub enum EndpointSecurity {
	Insecure,
	#[ cfg (feature = "hss-tls-rust") ]
	RustTls (RustTlsCertificate),
	#[ cfg (feature = "hss-tls-native") ]
	NativeTls (NativeTlsCertificate),
}


#[ derive (Clone) ]
#[ cfg (feature = "hss-tls-rust") ]
pub struct RustTlsCertificate {
	pub certified : rustls::sign::CertifiedKey,
}

#[ derive (Clone) ]
#[ cfg (feature = "hss-tls-native") ]
pub struct NativeTlsCertificate {
	pub identity : natls::Identity,
}




#[ cfg (feature = "hss-config") ]
impl Configuration {
	
	pub fn builder () -> ConfigurationBuilder {
		ConfigurationBuilder::new ()
	}
	
	pub fn localhost_http () -> ConfigurationBuilder {
		Configuration::builder ()
			.with_endpoint (Endpoint::localhost_http ())
	}
	
	#[ cfg (any (feature = "hss-tls-rust", feature = "hss-tls-native")) ]
	pub fn localhost_https () -> ConfigurationBuilder {
		Configuration::builder ()
			.with_endpoint (Endpoint::localhost_https ())
	}
}




#[ cfg (feature = "hss-config") ]
impl Default for Endpoint {
	
	fn default () -> Self {
		Endpoint {
				address : EndpointAddress::default (),
				#[ cfg (feature = "hyper--http") ]
				protocol : EndpointProtocol::default (),
				security : EndpointSecurity::default (),
			}
	}
}


#[ cfg (feature = "hss-config") ]
impl Default for EndpointAddress {
	
	fn default () -> Self {
		EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 0)))
	}
}


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hyper--http") ]
impl Default for EndpointProtocol {
	
	fn default () -> Self {
		
		#[ cfg (feature = "hyper--http1") ]
		#[ cfg (not (feature = "hyper--http2")) ]
		return EndpointProtocol::Http1;
		
		#[ cfg (feature = "hyper--http2") ]
		#[ cfg (not (feature = "hyper--http1")) ]
		return EndpointProtocol::Http2;
		
		#[ cfg (feature = "hyper--http1") ]
		#[ cfg (feature = "hyper--http2") ]
		return EndpointProtocol::Http12;
		
//		panic_with_message (0x783a0cf3, "no protocol enabled");
	}
}


#[ cfg (feature = "hss-config") ]
impl Default for EndpointSecurity {
	
	fn default () -> Self {
		EndpointSecurity::Insecure
	}
}




#[ cfg (feature = "hss-config") ]
impl Endpoint {
	
	pub fn localhost_http () -> Self {
		
		let mut _endpoint = Endpoint {
				.. Default::default ()
			};
		
		_endpoint.address = EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8080)));
		
		_endpoint
	}
	
	#[ cfg (any (feature = "hss-tls-rust", feature = "hss-tls-native")) ]
	pub fn localhost_https () -> Self {
		
		let _security = EndpointSecurity::Insecure;
		#[ cfg (feature = "hss-tls-rust") ]
		let _security = {
			let _certificate = RustTlsCertificate::localhost () .or_panic (0xf64b30c4);
			EndpointSecurity::RustTls (_certificate)
		};
		#[ cfg (feature = "hss-tls-native") ]
		let _security = {
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




#[ cfg (feature = "hss-config") ]
pub struct ConfigurationBuilder {
	endpoint : Option<Endpoint>,
	#[ cfg (feature = "hss-handler") ]
	handler : Option<HandlerDynArc>,
	#[ cfg (feature = "hss-routes") ]
	routes : Option<RoutesBuilder>,
}


#[ cfg (feature = "hss-config") ]
impl ConfigurationBuilder {
	
	pub fn new () -> Self {
		Self {
				endpoint : None,
				#[ cfg (feature = "hss-handler") ]
				handler : None,
				#[ cfg (feature = "hss-routes") ]
				routes : None,
			}
	}
	
	pub fn build (self) -> ServerResult<Configuration> {
		
		let ConfigurationBuilder {
				endpoint : _endpoint,
				#[ cfg (feature = "hss-handler") ]
				handler : _handler,
				#[ cfg (feature = "hss-routes") ]
				routes : _routes,
			} = self;
		
		let _endpoint = if let Some (_endpoint) = _endpoint {
			_endpoint
		} else {
			Endpoint::default ()
		};
		
		#[ cfg (feature = "hss-handler") ]
		#[ cfg (feature = "hss-routes") ]
		if _handler.is_some () && _routes.is_some () {
			return Err (error_with_message (0xc7d24cd3, "both handler and routes specified"))
		}
		
		#[ cfg (feature = "hss-handler") ]
		let mut _handler_0 = None;
		#[ cfg (feature = "hss-handler") ]
		if _handler_0.is_none () {
			if let Some (_handler) = _handler {
				_handler_0 = Some (_handler);
			}
		}
		#[ cfg (feature = "hss-routes") ]
		if _handler_0.is_none () {
			if let Some (_routes) = _routes {
				_handler_0 = Some (_routes.build () ? .into_boxed ());
			}
		}
		
		let _configuration = Configuration {
				endpoint : _endpoint,
				#[ cfg (feature = "hss-handler") ]
				handler : _handler_0,
			};
		
		Ok (_configuration)
	}
}


#[ cfg (feature = "hss-config") ]
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
	
	#[ cfg (feature = "hyper--http") ]
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


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-tls-rust") ]
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


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-tls-native") ]
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


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-handler") ]
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


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-routes") ]
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




#[ cfg (feature = "hss-tls-rust") ]
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




#[ cfg (feature = "hss-tls-native") ]
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

