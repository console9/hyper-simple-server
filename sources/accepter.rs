

use crate::prelude::*;




#[ cfg (feature = "hss-accepter") ]
pub enum Accepter {
	TcpListener (Arc<tokio::TcpListener>, Arc<hyper::Http>),
	#[ cfg (feature = "hss-tls-rust") ]
	RustTlsTcpListener (Arc<tokio_rustls::TlsAcceptor>, Arc<tokio::TcpListener>, Arc<hyper::Http>),
	#[ cfg (feature = "hss-tls-native") ]
	NativeTlsTcpListener (Arc<tokio_natls::TlsAcceptor>, Arc<tokio::TcpListener>, Arc<hyper::Http>),
}




#[ cfg (feature = "hss-accepter") ]
impl hyper::Accept for Accepter {
	
	type Conn = Connection;
	type Error = ServerError;
	
	fn poll_accept (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
		
		let _self = Pin::into_inner (self);
		
		let _listener = _self.listener ();
		let (_socket, _address) = match futures::ready! (_listener.poll_accept (_context)) {
			Ok ((_socket, _address)) =>
				(_socket, _address),
			Err (_error) =>
				return Poll::Ready (Some (Err (_error))),
		};
		
		match _self {
			
			Accepter::TcpListener (_, _) => {
				let _connection = Connection::TcpStream (_socket, _address);
				Poll::Ready (Some (Ok (_connection)))
			}
			
			#[ cfg (feature = "hss-tls-rust") ]
			Accepter::RustTlsTcpListener (_tls, _, _) => {
				let _accepter = _tls.accept (_socket);
				let _connection = Connection::RustTlsTcpStreamPending (_accepter, _address);
				Poll::Ready (Some (Ok (_connection)))
			}
			
			#[ cfg (feature = "hss-tls-native") ]
			Accepter::NativeTlsTcpListener (_tls, _, _) => {
				let _tls = _tls.clone ();
				#[ allow (unsafe_code) ]
				let _tls_static = unsafe { mem::transmute::<&tokio_natls::TlsAcceptor, &'static tokio_natls::TlsAcceptor> (_tls.deref ()) };
				let _accepter = _tls_static.accept (_socket);
				let _accepter = Box::pin (_accepter);
				let _connection = Connection::NativeTlsTcpStreamPending (_tls, _accepter, _address);
				Poll::Ready (Some (Ok (_connection)))
			}
		}
	}
}


#[ cfg (feature = "hss-accepter") ]
impl Accepter {
	
	pub fn new (_endpoint : &Endpoint) -> ServerResult<Self> {
		
		let _http = new_protocol (&_endpoint.protocol) ?;
		let _listener = new_listener (&_endpoint.address) ?;
		
		let _http = Arc::new (_http);
		let _listener = Arc::new (_listener);
		
		match &_endpoint.security {
			
			EndpointSecurity::Insecure =>
				Ok (Accepter::TcpListener (_listener, _http)),
			
			#[ cfg (feature = "hss-tls-rust") ]
			EndpointSecurity::RustTls (_certificate) => {
				let _accepter = new_rustls_accepter (_certificate, &_endpoint.protocol) ?;
				Ok (Accepter::RustTlsTcpListener (Arc::new (_accepter), _listener, _http))
			}
			
			#[ cfg (feature = "hss-tls-native") ]
			EndpointSecurity::NativeTls (_certificate) => {
				let _accepter = new_native_accepter (_certificate, &_endpoint.protocol) ?;
				Ok (Accepter::NativeTlsTcpListener (Arc::new (_accepter), _listener, _http))
			}
		}
	}
	
	pub(crate) fn protocol (&self) -> Arc<hyper::Http> {
		match self {
			Accepter::TcpListener (_, _protocol) =>
				_protocol.clone (),
			#[ cfg (feature = "hss-tls-rust") ]
			Accepter::RustTlsTcpListener (_, _, _protocol) =>
				_protocol.clone (),
			#[ cfg (feature = "hss-tls-native") ]
			Accepter::NativeTlsTcpListener (_, _, _protocol) =>
				_protocol.clone (),
		}
	}
	
	pub(crate) fn listener (&self) -> &tokio::TcpListener {
		match self {
			Accepter::TcpListener (_listener, _) =>
				_listener,
			#[ cfg (feature = "hss-tls-rust") ]
			Accepter::RustTlsTcpListener (_, _listener, _) =>
				_listener,
			#[ cfg (feature = "hss-tls-native") ]
			Accepter::NativeTlsTcpListener (_, _listener, _) =>
				_listener,
		}
	}
}




#[ cfg (feature = "hss-accepter") ]
fn new_protocol (_protocol : &EndpointProtocol) -> ServerResult<hyper::Http> {
	
	let mut _http = hyper::Http::new ();
	
	match _protocol {
		#[ cfg (feature = "hyper--http1") ]
		EndpointProtocol::Http1 => {
			_http.http1_only (true);
		}
		#[ cfg (feature = "hyper--http2") ]
		EndpointProtocol::Http2 => {
			_http.http2_only (true);
		}
		#[ cfg (feature = "hyper--http1") ]
		#[ cfg (feature = "hyper--http2") ]
		EndpointProtocol::Http12 =>
			(),
	}
	
	#[ cfg (feature = "hyper--http1") ]
	{
		let _apply = match _protocol {
			EndpointProtocol::Http1 => true,
			#[ cfg (feature = "hyper--http2") ]
			EndpointProtocol::Http12 => true,
			_ => false,
		};
		if _apply {
			_http.http1_keep_alive (true);
			_http.http1_half_close (true);
			_http.max_buf_size (16 * 1024);
		}
	}
	
	#[ cfg (feature = "hyper--http2") ]
	{
		let _apply = match _protocol {
			EndpointProtocol::Http2 => true,
			#[ cfg (feature = "hyper--http1") ]
			EndpointProtocol::Http12 => true,
			_ => false,
		};
		if _apply {
			_http.http2_max_concurrent_streams (128);
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_interval (Some (time::Duration::new (6, 0)));
			#[ cfg (feature = "hyper--runtime") ]
			_http.http2_keep_alive_timeout (time::Duration::new (30, 0));
		}
	}
	
	Ok (_http)
}




#[ allow (unsafe_code) ]
#[ cfg (feature = "hss-accepter") ]
fn new_listener (_address : &EndpointAddress) -> ServerResult<tokio::TcpListener> {
	
	let _listener = match _address {
		EndpointAddress::Socket (_address) =>
			net::TcpListener::bind (_address) ?,
		EndpointAddress::Descriptor (_descriptor) =>
			unsafe {
				os::unix::io::FromRawFd::from_raw_fd (*_descriptor as i32)
			},
	};
	
	let _listener = tokio::TcpListener::from_std (_listener) ?;
	
	Ok (_listener)
}




#[ cfg (feature = "hss-accepter") ]
#[ cfg (feature = "hss-tls-rust") ]
fn new_rustls_accepter (_certificate : &RustTlsCertificate, _protocol : &EndpointProtocol) -> ServerResult<tokio_rustls::TlsAcceptor> {
	
	let _resolver = {
		struct Resolver (RustTlsCertificate);
		impl rustls::ResolvesServerCert for Resolver {
			fn resolve (&self, _ : rustls::ClientHello<'_>) -> Option<rustls::sign::CertifiedKey> {
				Some (self.0.certified.clone ())
			}
		}
		Resolver (_certificate.clone ())
	};
	
	let _configuration = {
		let mut _builder = rustls::ServerConfig::new (rustls::NoClientAuth::new ());
		_builder.cert_resolver = Arc::new (_resolver);
		match _protocol {
			EndpointProtocol::Http1 =>
				_builder.alpn_protocols.push ("http/1.1".into ()),
			EndpointProtocol::Http2 =>
				_builder.alpn_protocols.push ("h2".into ()),
			EndpointProtocol::Http12 => {
				_builder.alpn_protocols.push ("h2".into ());
				_builder.alpn_protocols.push ("http/1.1".into ());
			}
		}
		Arc::new (_builder)
	};
	
	let _accepter = tokio_rustls::TlsAcceptor::from (_configuration);
	
	Ok (_accepter)
}




#[ cfg (feature = "hss-accepter") ]
#[ cfg (feature = "hss-tls-native") ]
fn new_native_accepter (_certificate : &NativeTlsCertificate, _protocol : &EndpointProtocol) -> ServerResult<tokio_natls::TlsAcceptor> {
	
	let _configuration = {
		let mut _builder = natls::TlsAcceptor::builder (_certificate.identity.clone ());
		_builder.min_protocol_version (Some (natls::Protocol::Tlsv12));
		_builder.build () .or_wrap (0xaf2c7136) ?
	};
	
	let _accepter = tokio_natls::TlsAcceptor::from (_configuration);
	
	Ok (_accepter)
}

