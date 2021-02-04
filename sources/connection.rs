

use crate::prelude::*;




pub enum Connection {
	TcpStream (tokio::TcpStream, net::SocketAddr),
	RustTlsTcpStreamPending (tokio_rustls::Accept<tokio::TcpStream>, net::SocketAddr),
	RustTlsTcpStream (tokio_rustls::server::TlsStream<tokio::TcpStream>, net::SocketAddr),
}




impl Connection {
	
	fn poll_stream (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult<Pin<&mut dyn AsyncStream>>> {
		
		let _self = Pin::into_inner (self);
		
		match _self {
			
			Connection::TcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
			
			Connection::RustTlsTcpStreamPending (_accepter, _address) =>
				match futures::ready! (Pin::new (_accepter).poll (_context)) {
					Ok (_stream) => {
						*_self = Connection::RustTlsTcpStream (_stream, *_address);
						Self::poll_stream (Pin::new (_self), _context)
					}
					Err (_error) =>
						Poll::Ready (Err (_error)),
				}
			
			Connection::RustTlsTcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
		}
	}
}

trait AsyncStream : tokio::AsyncRead + tokio::AsyncWrite + Unpin {}
impl <Stream : tokio::AsyncRead + tokio::AsyncWrite + Unpin> AsyncStream for Stream {}




impl tokio::AsyncRead for Connection {
	
	fn poll_read (self : Pin<&mut Self>, _context : &mut Context<'_>, _buffer : &mut tokio::ReadBuf<'_>) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_read (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}


impl tokio::AsyncWrite for Connection {
	
	fn poll_write (self : Pin<&mut Self>, _context : &mut Context<'_>, _buffer : &[u8]) -> Poll<ServerResult<usize>> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_write (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_flush (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_flush (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_shutdown (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_shutdown (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}
