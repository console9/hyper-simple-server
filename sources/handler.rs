

use crate::prelude::*;




pub type Request = hyper::Request<Body>;
pub type Response = hyper::Response<Body>;
pub type Body = hyper::Body;




pub trait Handler : Sized + Send + Sync + 'static {
	
	type Future : Future<Output = ServerResult<Response>> + Send + 'static;
	
	fn handle (&self, _request : Request) -> Self::Future;
	
	fn into_boxed (self) -> HandlerDynArc {
		HandlerDynArc (Arc::new (self))
	}
}




pub struct HandlerFnAsync <C, F> (C)
		where
			C : Fn (Request) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response>> + Send + 'static
;

impl <C, F> Handler for HandlerFnAsync<C, F>
		where
			C : Fn (Request) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response>> + Send + 'static
{
	type Future = F;
	
	fn handle (&self, _request : Request) -> Self::Future {
		self.0 (_request)
	}
}

impl <C, F> From<C> for HandlerFnAsync<C, F>
		where
			C : Fn (Request) -> F + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response>> + Send + 'static
{
	fn from (_fn : C) -> Self {
		Self (_fn)
	}
}




pub struct HandlerFnSync <C> (C)
		where
			C : Fn (Request) -> ServerResult<Response> + Send + Sync + 'static
;

impl <C> Handler for HandlerFnSync<C>
		where
			C : Fn (Request) -> ServerResult<Response> + Send + Sync + 'static
{
	type Future = future::Ready<ServerResult<Response>>;
	
	fn handle (&self, _request : Request) -> Self::Future {
		future::ready (self.0 (_request))
	}
}

impl <C> From<C> for HandlerFnSync<C>
		where
			C : Fn (Request) -> ServerResult<Response> + Send + Sync + 'static
{
	fn from (_fn : C) -> Self {
		Self (_fn)
	}
}




pub trait HandlerDyn : Send + Sync + 'static {
	
	fn handle (&self, _request : Request) -> Pin<Box<dyn Future<Output = ServerResult<Response>> + Send>>;
}


impl <H, F> HandlerDyn for H
		where
			H : Handler<Future = F> + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response>> + Send + 'static
{
	fn handle (&self, _request : Request) -> Pin<Box<dyn Future<Output = ServerResult<Response>> + Send>> {
		let _future = Handler::handle (self, _request);
		Box::pin (_future)
	}
}

impl <H, F> From<H> for HandlerDynArc
		where
			H : Handler<Future = F> + Send + Sync + 'static,
			F : Future<Output = ServerResult<Response>> + Send + 'static
{
	fn from (_handler : H) -> Self {
		_handler.into_boxed ()
	}
}




#[ derive (Clone) ]
pub struct HandlerDynArc (Arc<dyn HandlerDyn>);


impl hyper::Service<Request> for HandlerDynArc {
	
	type Response = Response;
	type Error = ServerError;
	type Future = Pin<Box<dyn Future<Output = ServerResult<Response>> + Send>>;
	
	
	fn poll_ready (&mut self, _context : &mut Context) -> Poll<ServerResult> {
		Poll::Ready (Ok (()))
	}
	
	fn call (&mut self, _request : Request) -> Self::Future {
		self.0.handle (_request)
	}
}

