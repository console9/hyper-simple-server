

use crate::prelude::*;




#[ cfg (feature = "hss-handler") ]
pub trait Handler
	where
		Self : Send + Sync + 'static,
{
	type Future : Future<Output = ServerResult<Response<Self::ResponseBody>>> + Send + 'static;
	type ResponseBody : BodyTrait<Data = Bytes, Error = Self::ResponseBodyError> + Send + 'static;
	type ResponseBodyError : Error + Send + Sync + 'static;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future;
	
	fn wrap (self) -> HandlerWrapper<Self> where Self : Sized {
		HandlerWrapper (self)
	}
}




pub struct HandlerWrapper <H : Handler> (H);

#[ cfg (feature = "hss-handler") ]
impl <H> hyper::Service<Request<Body>> for HandlerWrapper<H>
	where
		H : Handler,
{
	type Future = H::Future;
	type Response = Response<H::ResponseBody>;
	type Error = ServerError;
	
	fn poll_ready (&mut self, _context : &mut Context<'_>) -> Poll<ServerResult> {
		Poll::Ready (Ok (()))
	}
	
	fn call (&mut self, _request : Request<Body>) -> Self::Future {
		self.0.handle (_request)
	}
}




#[ cfg (feature = "hss-handler") ]
pub struct HandlerFnAsync <C, F, RB>
	where
		C : Fn (Request<Body>) -> F + Send + Sync + 'static,
		F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	function : C,
	phantom : PhantomData<fn(RB)>,
}


#[ cfg (feature = "hss-handler") ]
impl <C, F, RB> Handler for HandlerFnAsync<C, F, RB>
	where
		C : Fn (Request<Body>) -> F + Send + Sync + 'static,
		F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	type Future = F;
	type ResponseBody = RB;
	type ResponseBodyError = RB::Error;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		(self.function) (_request)
	}
}


#[ cfg (feature = "hss-handler") ]
impl <C, F, RB> From<C> for HandlerFnAsync<C, F, RB>
	where
		C : Fn (Request<Body>) -> F + Send + Sync + 'static,
		F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	fn from (_function : C) -> Self {
		Self {
				function : _function,
				phantom : PhantomData,
			}
	}
}




#[ cfg (feature = "hss-handler") ]
pub struct HandlerFnSync <C, RB>
	where
		C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	function : C,
	phantom : PhantomData<fn(RB)>,
}


#[ cfg (feature = "hss-handler") ]
impl <C, RB> Handler for HandlerFnSync<C, RB>
	where
		C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	type Future = future::Ready<ServerResult<Response<RB>>>;
	type ResponseBody = RB;
	type ResponseBodyError = RB::Error;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		future::ready ((self.function) (_request))
	}
}


#[ cfg (feature = "hss-handler") ]
impl <C, RB> From<C> for HandlerFnSync<C, RB>
	where
		C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
		RB : BodyTrait<Data = Bytes> + Send + 'static,
		RB::Error : Error + Send + Sync + 'static,
{
	fn from (_function : C) -> Self {
		Self {
				function : _function,
				phantom : PhantomData,
			}
	}
}




#[ cfg (feature = "hss-handler") ]
pub trait HandlerDyn
	where
		Self : Send + Sync + 'static,
{
	fn handle (&self, _request : Request<Body>) -> HandlerFutureDynBox;
}


#[ cfg (feature = "hss-handler") ]
impl <H> HandlerDyn for H
	where
		H : Handler + Send + Sync + 'static,
		H::Future : Future<Output = ServerResult<Response<H::ResponseBody>>> + Send + 'static,
{
	fn handle (&self, _request : Request<Body>) -> HandlerFutureDynBox {
		let _future = Handler::handle (self, _request);
		let _future = _future.map_ok (|_response| _response.map (BodyDynBox::new));
		HandlerFutureDynBox::new (_future)
	}
}




#[ derive (Clone) ]
#[ cfg (feature = "hss-handler") ]
pub struct HandlerDynArc (Arc<dyn HandlerDyn>);


#[ cfg (feature = "hss-handler") ]
impl HandlerDynArc {
	
	pub fn new (_handler : impl HandlerDyn) -> Self {
		HandlerDynArc (Arc::new (_handler))
	}
	
	pub fn delegate (&self, _request : Request<Body>) -> HandlerFutureDynBox {
		self.0.handle (_request)
	}
}


#[ cfg (feature = "hss-handler") ]
impl Handler for HandlerDynArc {
	
	type Future = HandlerFutureDynBox;
	type ResponseBody = BodyDynBox;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		self.delegate (_request)
	}
}




#[ cfg (feature = "hss-handler") ]
pub struct HandlerFutureDynBox (Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>>);


#[ cfg (feature = "hss-handler") ]
impl Future for HandlerFutureDynBox {
	
	type Output = ServerResult<Response<BodyDynBox>>;
	
	fn poll (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Self::Output> {
		let _self = Pin::into_inner (self);
		_self.0.as_mut () .poll (_context)
	}
}


#[ cfg (feature = "hss-handler") ]
impl HandlerFutureDynBox {
	
	pub fn new <F> (_future : F) -> Self
			where
				F : Future<Output = ServerResult<Response<BodyDynBox>>> + Send + 'static
	{
		Self (Box::pin (_future))
	}
	
	pub fn ready (_result : ServerResult<Response<BodyDynBox>>) -> Self {
		Self::new (future::ready (_result))
	}
	
	pub fn ready_response (_response : Response<BodyDynBox>) -> Self {
		Self::ready (Ok (_response))
	}
	
	pub fn ready_error (_error : ServerError) -> Self {
		Self::ready (Err (_error))
	}
}


#[ cfg (feature = "hss-handler") ]
impl <B> From<Response<B>> for HandlerFutureDynBox
	where
		B : BodyTrait<Data = Bytes> + Send + 'static,
		B::Error : Error + Send + Sync + 'static,
{
	fn from (_response : Response<B>) -> Self {
		Self::ready_response (_response.map (BodyDynBox::new))
	}
}


#[ cfg (feature = "hss-handler") ]
impl From<ServerError> for HandlerFutureDynBox {
	fn from (_error : ServerError) -> Self {
		Self::ready_error (_error)
	}
}




#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub trait HandlerSimpleAsync
	where
		Self : Send + Sync + 'static,
{
	type Future : Future<Output = ServerResult<Response<Body>>> + Send + 'static;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future;
	
	fn wrap (self) -> HandlerSimpleAsyncWrapper<Self> where Self : Sized {
		HandlerSimpleAsyncWrapper (self)
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct HandlerSimpleAsyncWrapper <H> (H)
	where
		H : HandlerSimpleAsync,
;


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
impl <H> Handler for HandlerSimpleAsyncWrapper<H>
	where
		H : HandlerSimpleAsync,
		H::Future : Unpin,
{
	type Future = HandlerSimpleAsyncWrapperFuture<H::Future>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let _future = HandlerSimpleAsync::handle (&self.0, _request);
		let _future = HandlerSimpleAsyncWrapperFuture (_future);
		_future
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct HandlerSimpleAsyncWrapperFuture <F> (F)
	where
		F : Future<Output = ServerResult<Response<Body>>> + Send + 'static + Unpin,
;

#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
impl <F> Future for HandlerSimpleAsyncWrapperFuture<F>
	where
		F : Future<Output = ServerResult<Response<Body>>> + Send + 'static + Unpin,
{
	type Output = ServerResult<Response<BodyWrapper<Body>>>;
	
	fn poll (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Self::Output> {
		let _self = Pin::into_inner (self);
		let _delegate = Pin::new (&mut _self.0);
		let _poll = _delegate.poll (_context);
		let _poll = _poll.map_ok (|_response| _response.map (BodyWrapper::new));
		_poll
	}
}




#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub trait HandlerSimpleSync
	where
		Self : Send + Sync + 'static,
{
	fn handle (&self, _request : &Request<Body>, _response : &mut Response<Body>) -> ServerResult;
	
	fn wrap (self) -> HandlerSimpleSyncWrapper<Self> where Self : Sized {
		HandlerSimpleSyncWrapper (self)
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct HandlerSimpleSyncWrapper <H> (H)
	where
		H : HandlerSimpleSync,
;


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
impl <H> Handler for HandlerSimpleSyncWrapper<H>
	where
		H : HandlerSimpleSync,
{
	type Future = future::Ready<ServerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let mut _response = Response::new (Body::empty ());
		match HandlerSimpleSync::handle (&self.0, &_request, &mut _response) {
			Ok (()) =>
				future::ready (Ok (_response.map (BodyWrapper::new))),
			Err (_error) =>
				future::ready (Err (_error)),
		}
	}
}




#[ cfg (feature = "hss-handler") ]
pub struct BodyWrapper <B> (B)
	where
		B : BodyTrait<Data = Bytes> + Send + 'static + Unpin,
		B::Error : Error + Send + Sync + 'static,
;


#[ cfg (feature = "hss-handler") ]
impl <B> BodyTrait for BodyWrapper<B>
	where
		B : BodyTrait<Data = Bytes> + Send + 'static + Unpin,
		B::Error : Error + Send + Sync + 'static,
{
	type Data = Bytes;
	type Error = ServerError;
	
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Option<ServerResult<Bytes>>> {
		let _future = self.delegate_pin_mut () .poll_data (_context);
		let _future = _future.map (|_option| _option.map (|_result| _result.map_err (|_error| _error.wrap (0x4e33a117))));
		_future
	}
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult<Option<Headers>>> {
		let _future = self.delegate_pin_mut () .poll_trailers (_context);
		let _future = _future.map (|_result| _result.map_err (|_error| _error.wrap (0x3a25b983)));
		_future
	}
	
	fn is_end_stream (&self) -> bool {
		self.delegate () .is_end_stream ()
	}
	
	fn size_hint (&self) -> BodySizeHint {
		self.delegate () .size_hint ()
	}
}


#[ cfg (feature = "hss-handler") ]
impl <B> BodyWrapper<B>
	where
		B : BodyTrait<Data = Bytes> + Send + 'static + Unpin,
		B::Error : Error + Send + Sync + 'static,
{
	pub fn new (_body : B) -> Self {
		Self (_body)
	}
	
	fn delegate_pin_mut (self : Pin<&mut Self>) -> Pin<&mut B> {
		let _self = Pin::into_inner (self);
		Pin::new (&mut _self.0)
	}
	
	fn delegate (&self) -> Pin<&B> {
		Pin::new (&self.0)
	}
}




#[ cfg (feature = "hss-handler") ]
pub trait BodyDyn
	where
		Self : Send + 'static,
{
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Option<ServerResult<Bytes>>>;
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult<Option<Headers>>>;
	
	fn is_end_stream (&self) -> bool;
	fn size_hint (&self) -> BodySizeHint;
}


#[ cfg (feature = "hss-handler") ]
impl <B> BodyDyn for B
	where
		B : BodyTrait<Data = Bytes> + Send + 'static,
		B::Error : Error + Send + Sync + 'static,
{
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Option<ServerResult<Bytes>>> {
		let _future = BodyTrait::poll_data (self, _context);
		let _future = _future.map (|_option| _option.map (|_result| _result.map_err (|_error| _error.wrap (0xd89897d4))));
		_future
	}
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult<Option<Headers>>> {
		let _future = BodyTrait::poll_trailers (self, _context);
		let _future = _future.map (|_result| _result.map_err (|_error| _error.wrap (0x8adea6a0)));
		_future
	}
	
	fn is_end_stream (&self) -> bool {
		BodyTrait::is_end_stream (self)
	}
	
	fn size_hint (&self) -> BodySizeHint {
		BodyTrait::size_hint (self)
	}
}




#[ cfg (feature = "hss-handler") ]
pub struct BodyDynBox (Pin<Box<dyn BodyDyn>>);


#[ cfg (feature = "hss-handler") ]
impl BodyTrait for BodyDynBox {
	
	type Data = Bytes;
	type Error = ServerError;
	
	fn poll_data (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<Option<ServerResult<Bytes>>> {
		self.delegate_pin_mut () .poll_data (_context)
	}
	
	fn poll_trailers (self : Pin<&mut Self>, _context : &mut Context<'_>) -> Poll<ServerResult<Option<Headers>>> {
		self.delegate_pin_mut () .poll_trailers (_context)
	}
	
	fn is_end_stream (&self) -> bool {
		self.delegate () .is_end_stream ()
	}
	
	fn size_hint (&self) -> BodySizeHint {
		self.delegate () .size_hint ()
	}
}


#[ cfg (feature = "hss-handler") ]
impl BodyDynBox {
	
	pub fn new (_body : impl BodyDyn) -> Self {
		Self (Box::pin (_body))
	}
	
	fn delegate_pin_mut (self : Pin<&mut Self>) -> Pin<&mut dyn BodyDyn> {
		let _self = Pin::into_inner (self);
		_self.0.as_mut ()
	}
	
	fn delegate (&self) -> Pin<&dyn BodyDyn> {
		self.0.as_ref ()
	}
}

