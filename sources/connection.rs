

use crate::prelude::*;




#[ cfg (feature = "hss-accepter") ]
pub enum Connection {
	TcpStream (tokio::TcpStream, net::SocketAddr),
	#[ cfg (feature = "hss-tls-rust") ]
	RustTlsTcpStreamPending (tokio_rustls::Accept<tokio::TcpStream>, net::SocketAddr),
	#[ cfg (feature = "hss-tls-rust") ]
	RustTlsTcpStream (tokio_rustls::server::TlsStream<tokio::TcpStream>, net::SocketAddr),
	#[ cfg (feature = "hss-tls-native") ]
	NativeTlsTcpStreamPending (Arc<tokio_natls::TlsAcceptor>, Pin<Box<dyn Future<Output = Result<tokio_natls::TlsStream<tokio::TcpStream>, natls::Error>> + Send + 'static>>, net::SocketAddr),
	#[ cfg (feature = "hss-tls-native") ]
	NativeTlsTcpStream (tokio_natls::TlsStream<tokio::TcpStream>, net::SocketAddr),
}




#[ cfg (feature = "hss-accepter") ]
impl Connection {
	
	fn poll_stream (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<StdIoResult<Pin<&mut dyn AsyncStream>>> {
		
		let _self = Pin::into_inner (self);
		
		match _self {
			
			Connection::TcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
			
			#[ cfg (feature = "hss-tls-rust") ]
			Connection::RustTlsTcpStreamPending (_accepter, _address) =>
				match futures::ready! (Pin::new (_accepter) .poll (_context)) {
					Ok (_stream) => {
						*_self = Connection::RustTlsTcpStream (_stream, *_address);
						Self::poll_stream (Pin::new (_self), _context)
					}
					Err (_error) =>
						Poll::Ready (Err (_error)),
				}
			
			#[ cfg (feature = "hss-tls-rust") ]
			Connection::RustTlsTcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
			
			#[ cfg (feature = "hss-tls-native") ]
			Connection::NativeTlsTcpStreamPending (_tls, _accepter, _address) =>
				match futures::ready! (_accepter.as_mut () .poll (_context)) {
					Ok (_stream) => {
						*_self = Connection::NativeTlsTcpStream (_stream, *_address);
						Self::poll_stream (Pin::new (_self), _context)
					}
					Err (_error) =>
						Poll::Ready (Err (_error.wrap (0xba9facee))),
				}
			
			#[ cfg (feature = "hss-tls-native") ]
			Connection::NativeTlsTcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
		}
	}
}


#[ cfg (feature = "hss-accepter") ]
trait AsyncStream : tokio::AsyncRead + tokio::AsyncWrite + Unpin {}

#[ cfg (feature = "hss-accepter") ]
impl <S : tokio::AsyncRead + tokio::AsyncWrite + Unpin> AsyncStream for S {}




#[ cfg (feature = "hss-accepter") ]
impl tokio::AsyncRead for Connection {
	
	fn poll_read (self : Pin<&mut Self>, _context : &mut Context<'_>, _buffer : &mut tokio::ReadBuf<'_>) -> Poll<StdIoResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_read (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}


#[ cfg (feature = "hss-accepter") ]
impl tokio::AsyncWrite for Connection {
	
	fn poll_write (self : Pin<&mut Self>, _context : &mut Context<'_>, _buffer : &[u8]) -> Poll<StdIoResult<usize>> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_write (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_flush (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<StdIoResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_flush (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_shutdown (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<StdIoResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_shutdown (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}

