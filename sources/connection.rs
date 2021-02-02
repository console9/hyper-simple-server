

use crate::prelude::*;




pub(crate) enum Connection {
	TcpStream (tokio::TcpStream, net::SocketAddr),
}




impl Connection {
	
	fn poll_stream (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult<Pin<&mut dyn AsyncStream>>> {
		
		let _self = Pin::into_inner (self);
		
		match _self {
			Connection::TcpStream (_stream, _) =>
				Poll::Ready (Ok (Pin::new (_stream))),
		}
	}
}

trait AsyncStream : tokio::AsyncRead + tokio::AsyncWrite + Unpin {}
impl <Stream : tokio::AsyncRead + tokio::AsyncWrite + Unpin> AsyncStream for Stream {}




impl tokio::AsyncRead for Connection {
	
	fn poll_read (self : Pin<&mut Self>, _context : &mut Context, _buffer : &mut tokio::ReadBuf) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_read (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}


impl tokio::AsyncWrite for Connection {
	
	fn poll_write (self : Pin<&mut Self>, _context : &mut Context, _buffer : &[u8]) -> Poll<ServerResult<usize>> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_write (_context, _buffer),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_flush (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_flush (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
	
	fn poll_shutdown (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult> {
		match futures::ready! (self.poll_stream (_context)) {
			Ok (_stream) =>
				_stream.poll_shutdown (_context),
			Err (_error) =>
				Poll::Ready (Err (_error)),
		}
	}
}

