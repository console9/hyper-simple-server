

use crate::prelude::*;




pub use hyper::Request as Request;
pub use hyper::Response as Response;

pub use hyper::Body as Body;
pub use hyper::BodyTrait as BodyTrait;
pub use hyper::BodySizeHint as BodySizeHint;
pub use hyper::BodyData as BodyData;
pub use hyper::BodyTrailers as BodyTrailers;

pub use hyper::HeaderValue as HeaderValue;
pub type Headers = hyper::HeaderMap<HeaderValue>;

pub use hyper::Bytes as Bytes;




pub trait Handler : Send + Sync + 'static {
	
	type Future : Future<Output = ServerResult<Response<Self::ResponseBody>>> + Send + 'static;
	type ResponseBody : BodyTrait<Data = Bytes, Error = Self::ResponseBodyError> + Send + 'static;
	type ResponseBodyError : Error + Send + 'static;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future;
	
	fn into_boxed (self) -> HandlerDynArc where Self : Sized {
		HandlerDynArc (Arc::new (self))
	}
}




pub struct HandlerFnAsync <C, F, RB, RBE>
		where
			C : Fn (Request<Body>) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	function : C,
	phantom : PhantomData<fn(RB, RBE)>,
}


impl <C, F, RB, RBE> Handler for HandlerFnAsync<C, F, RB, RBE>
		where
			C : Fn (Request<Body>) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	type Future = F;
	type ResponseBody = RB;
	type ResponseBodyError = RBE;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		(self.function) (_request)
	}
}


impl <C, F, RB, RBE> From<C> for HandlerFnAsync<C, F, RB, RBE>
		where
			C : Fn (Request<Body>) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	fn from (_function : C) -> Self {
		Self {
				function : _function,
				phantom : PhantomData,
			}
	}
}




pub struct HandlerFnSync <C, RB, RBE>
		where
			C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	function : C,
	phantom : PhantomData<fn(RB, RBE)>,
}


impl <C, RB, RBE> Handler for HandlerFnSync<C, RB, RBE>
		where
			C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	type Future = future::Ready<ServerResult<Response<RB>>>;
	type ResponseBody = RB;
	type ResponseBodyError = RBE;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		future::ready ((self.function) (_request))
	}
}


impl <C, RB, RBE> From<C> for HandlerFnSync<C, RB, RBE>
		where
			C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
			RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
			RBE : Error + Send + 'static,
{
	fn from (_function : C) -> Self {
		Self {
				function : _function,
				phantom : PhantomData,
			}
	}
}




pub trait HandlerDyn : Send + Sync + 'static {
	
	fn handle (&self, _request : Request<Body>) -> Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>>;
}


impl <H, F> HandlerDyn for H
		where
			H : Handler<Future = F> + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
{
	fn handle (&self, _request : Request<Body>) -> Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>> {
		let _future = Handler::handle (self, _request);
		let _future = _future.map_ok (|_response| _response.map (BodyDynBox::from));
		Box::pin (_future)
	}
}


impl <H, F> From<H> for HandlerDynArc
		where
			H : Handler<Future = F> + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
{
	fn from (_handler : H) -> Self {
		_handler.into_boxed ()
	}
}




#[ derive (Clone) ]
pub struct HandlerDynArc (Arc<dyn HandlerDyn>);


impl HandlerDyn for HandlerDynArc {
	
	fn handle (&self, _request : Request<Body>) -> Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>> {
		self.0.handle (_request)
	}
}


impl hyper::Service<Request<Body>> for HandlerDynArc {
	
	type Future = Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>>;
	type Response = Response<BodyDynBox>;
	type Error = ServerError;
	
	fn poll_ready (&mut self, _context : &mut Context) -> Poll<ServerResult> {
		Poll::Ready (Ok (()))
	}
	
	fn call (&mut self, _request : Request<Body>) -> Self::Future {
		self.0.handle (_request)
	}
}




pub trait BodyDyn : Send + 'static {
	
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context) -> Poll<Option<ServerResult<Bytes>>>;
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult<Option<Headers>>>;
	
	fn is_end_stream (&self) -> bool;
	
	fn size_hint (&self) -> BodySizeHint;
}


impl <B, E> BodyDyn for B
		where
			B : BodyTrait<Data = Bytes, Error = E> + Send + 'static,
			E : Error + Send + 'static,
{
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context) -> Poll<Option<ServerResult<Bytes>>> {
		match futures::ready! (BodyTrait::poll_data (self, _context)) {
			Some (Ok (_bytes)) =>
				Poll::Ready (Some (Ok (_bytes))),
			Some (Err (_error)) =>
				Poll::Ready (Some (Err (_error.wrap (0xd89897d4)))),
			None =>
				Poll::Ready (None),
		}
	}
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult<Option<Headers>>> {
		match futures::ready! (BodyTrait::poll_trailers (self, _context)) {
			Ok (Some (_headers)) =>
				Poll::Ready (Ok (Some (_headers))),
			Ok (None) =>
				Poll::Ready (Ok (None)),
			Err (_error) =>
				Poll::Ready (Err (_error.wrap (0x8adea6a0))),
		}
	}
	
	fn is_end_stream (&self) -> bool {
		BodyTrait::is_end_stream (self)
	}
	
	fn size_hint (&self) -> BodySizeHint {
		BodyTrait::size_hint (self)
	}
}




pub struct BodyDynBox (Pin<Box<dyn BodyDyn>>);


impl BodyTrait for BodyDynBox {
	
	type Data = Bytes;
	type Error = ServerError;
	
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context) -> Poll<Option<ServerResult<Bytes>>> {
		self.delegate_pin_mut () .poll_data (_context)
	}
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context) -> Poll<ServerResult<Option<Headers>>> {
		self.delegate_pin_mut () .poll_trailers (_context)
	}
	
	fn is_end_stream (&self) -> bool {
		self.delegate () .is_end_stream ()
	}
	
	fn size_hint (&self) -> BodySizeHint {
		self.delegate () .size_hint ()
	}
}


impl BodyDynBox {
	
	fn delegate_pin_mut (self : Pin<&mut Self>) -> Pin<&mut dyn BodyDyn> {
		let _self = Pin::into_inner (self);
		_self.0.as_mut ()
	}
	
	fn delegate (&self) -> Pin<&dyn BodyDyn> {
		self.0.as_ref ()
	}
	
	fn from (_body : impl BodyDyn) -> Self {
		Self (Box::pin (_body))
	}
}

