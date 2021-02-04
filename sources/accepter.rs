

use crate::prelude::*;




pub enum Accepter {
	TcpListener (tokio::TcpListener, hyper::Http),
	RustTlsTcpListener (tokio_rustls::TlsAcceptor, tokio::TcpListener, hyper::Http),
}




impl hyper::Accept for Accepter {
	
	type Conn = Connection;
	type Error = ServerError;
	
	fn poll_accept (self : Pin<&mut Self>, _context : &mut Context) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
		
		match self.deref () {
			
			Accepter::TcpListener (_listener, _) =>
				match futures::ready! (_listener.poll_accept (_context)) {
					Ok ((_socket, _address)) => {
						let _connection = Connection::TcpStream (_socket, _address);
						Poll::Ready (Some (Ok (_connection)))
					}
					Err (_error) =>
						Poll::Ready (Some (Err (_error))),
				},
			
			Accepter::RustTlsTcpListener (_tls, _listener, _) =>
				match futures::ready! (_listener.poll_accept (_context)) {
					Ok ((_socket, _address)) => {
						let _accepter = _tls.accept (_socket);
						let _connection = Connection::RustTlsTcpStreamPending (_accepter, _address);
						Poll::Ready (Some (Ok (_connection)))
					}
					Err (_error) =>
						Poll::Ready (Some (Err (_error))),
				},
		}
	}
}


impl Accepter {
	
	pub(crate) fn new (_endpoint : &Endpoint) -> ServerResult<Self> {
		
		let _http = new_protocol (&_endpoint.protocol) ?;
		let _listener = new_listener (&_endpoint.address) ?;
		
		match &_endpoint.security {
			EndpointSecurity::Insecure =>
				Ok (Accepter::TcpListener (_listener, _http)),
			EndpointSecurity::RustTls (_certificate) => {
				let _accepter = new_rustls_accepter (_certificate, &_endpoint.protocol) ?;
				Ok (Accepter::RustTlsTcpListener (_accepter, _listener, _http))
			}
		}
	}
	
	pub(crate) fn protocol (&self) -> hyper::Http {
		match self {
			Accepter::TcpListener (_, _protocol) =>
				_protocol.clone (),
			Accepter::RustTlsTcpListener (_, _, _protocol) =>
				_protocol.clone (),
		}
	}
}




fn new_protocol (_protocol : &EndpointProtocol) -> ServerResult<hyper::Http> {
	
	let mut _http = hyper::Http::new ();
	
	match _protocol {
		EndpointProtocol::Http1 => {
			_http.http1_only (true);
		}
		EndpointProtocol::Http2 => {
			_http.http2_only (true);
		}
		EndpointProtocol::Http12 =>
			(),
	}
	
	match _protocol {
		EndpointProtocol::Http1 | EndpointProtocol::Http12 => {
			_http.http1_keep_alive (true);
			_http.http1_half_close (true);
			_http.max_buf_size (16 * 1024);
		}
		_ =>
			(),
	}
	
	match _protocol {
		EndpointProtocol::Http2 | EndpointProtocol::Http12 => {
			_http.http2_max_concurrent_streams (128);
			_http.http2_keep_alive_interval (Some (time::Duration::new (6, 0)));
			_http.http2_keep_alive_timeout (time::Duration::new (30, 0));
		}
		_ =>
			(),
	}
	
	Ok (_http)
}




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




fn new_rustls_accepter (_certificate : &RustTlsCertificate, _protocol : &EndpointProtocol) -> ServerResult<tokio_rustls::TlsAcceptor> {
	
	let _resolver = {
		struct Resolver (RustTlsCertificate);
		impl rustls::ResolvesServerCert for Resolver {
			fn resolve (&self, _ : rustls::ClientHello) -> Option<rustls::sign::CertifiedKey> {
				Some (self.0.certified.clone ())
			}
		}
		Resolver (_certificate.clone ())
	};
	
	let _configuration = {
		let mut _tls = rustls::ServerConfig::new (rustls::NoClientAuth::new ());
		_tls.cert_resolver = Arc::new (_resolver);
		match _protocol {
			EndpointProtocol::Http1 =>
				_tls.alpn_protocols.push ("http/1.1".into ()),
			EndpointProtocol::Http2 =>
				_tls.alpn_protocols.push ("h2".into ()),
			EndpointProtocol::Http12 => {
				_tls.alpn_protocols.push ("h2".into ());
				_tls.alpn_protocols.push ("http/1.1".into ());
			}
		}
		Arc::new (_tls)
	};
	
	let _accepter = tokio_rustls::TlsAcceptor::from (_configuration);
	
	Ok (_accepter)
}

