

use crate::prelude::*;




pub(crate) enum Accepter {
	TcpListener (tokio::TcpListener, hyper::Http),
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
		}
	}
}


impl Accepter {
	
	pub(crate) fn new (_endpoint : &Endpoint) -> ServerResult<Self> {
		
		let _http = hyper_new_protocol (&_endpoint.protocol) ?;
		let _listener = hyper_new_listener (&_endpoint.address) ?;
		
		Ok (Accepter::TcpListener (_listener, _http))
	}
	
	pub(crate) fn protocol (&self) -> hyper::Http {
		match self {
			Accepter::TcpListener (_, _protocol) =>
				_protocol.clone (),
		}
	}
}




fn hyper_new_protocol (_endpoint : &EndpointProtocol) -> ServerResult<hyper::Http> {
	
	let mut _http = hyper::Http::new ();
	
	match _endpoint {
		EndpointProtocol::Http1 => {
			_http.http1_only (true);
		}
		EndpointProtocol::Http2 => {
			_http.http2_only (true);
		}
		EndpointProtocol::Http12 =>
			(),
	}
	
	match _endpoint {
		EndpointProtocol::Http1 | EndpointProtocol::Http12 => {
			_http.http1_keep_alive (true);
			_http.http1_half_close (true);
			_http.max_buf_size (16 * 1024);
		}
		_ =>
			(),
	}
	
	match _endpoint {
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




fn hyper_new_listener (_endpoint : &EndpointAddress) -> ServerResult<tokio::TcpListener> {
	
	let _listener = match _endpoint {
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

