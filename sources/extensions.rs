

use crate::prelude::*;




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub trait RequestExt <B>
	where
		B : BodyTrait,
{
	fn is_get (&self) -> bool;
	fn is_head (&self) -> bool;
	fn is_put (&self) -> bool;
	fn is_post (&self) -> bool;
	fn is_delete (&self) -> bool;
	fn is_patch (&self) -> bool;
	fn is_options (&self) -> bool;
	
	
	fn uri_path (&self) -> &str;
	fn uri_query (&self) -> Option<&str>;
	
	fn header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue>;
	fn header_all (&self, _name : impl AsHeaderName) -> http::header::GetAll<'_, HeaderValue>;
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl <B> RequestExt<B> for Request<B>
	where
		B : BodyTrait
{
	fn is_get (&self) -> bool {
		self.method () == Method::GET
	}
	fn is_head (&self) -> bool {
		self.method () == Method::HEAD
	}
	fn is_put (&self) -> bool {
		self.method () == Method::PUT
	}
	fn is_post (&self) -> bool {
		self.method () == Method::POST
	}
	fn is_delete (&self) -> bool {
		self.method () == Method::DELETE
	}
	fn is_patch (&self) -> bool {
		self.method () == Method::PATCH
	}
	fn is_options (&self) -> bool {
		self.method () == Method::OPTIONS
	}
	
	fn uri_path (&self) -> &str {
		self.uri () .path ()
	}
	
	fn uri_query (&self) -> Option<&str> {
		self.uri () .query ()
	}
	
	fn header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue> {
		self.headers () .get (_name)
	}
	
	fn header_all (&self, _name : impl AsHeaderName) -> http::header::GetAll<'_, HeaderValue> {
		self.headers () .get_all (_name)
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub trait ResponseExt <B>
	where
		B : BodyTrait,
{
	fn set_status (&mut self, _status : StatusCode) -> &mut Self;
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	
	fn set_body (&mut self, _body : impl Into<B>) -> &mut Self;
	
	fn set_status_200 (&mut self) -> &mut Self {
		self.set_status (consts::OK)
	}
	
	fn set_content_type (&mut self, _content_type : impl Into<HeaderValue>) -> &mut Self {
		self.set_header (consts::CONTENT_TYPE, _content_type)
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl <B> ResponseExt<B> for Response<B>
	where
		B : BodyTrait,
{
	fn set_status (&mut self, _status : StatusCode) -> &mut Self {
		*self.status_mut () = _status;
		self
	}
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .insert (_name, _value.into ());
		self
	}
	
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .append (_name, _value.into ());
		self
	}
	
	fn set_body (&mut self, _body : impl Into<B>) -> &mut Self {
		*self.body_mut () = _body.into ();
		self
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub trait ResponseExtBuild <B>
	where
		B : BodyTrait,
		Self : Sized,
{
	fn new_with_status_and_body (_status : StatusCode, _content_type : Option<impl Into<HeaderValue>>, _body : impl Into<B>) -> Self;
	
	fn new_empty () -> Self where B : Default {
		let _content_type : Option<ContentType> = None;
		Self::new_with_status_and_body (consts::OK, _content_type, B::default ())
	}
	
	fn new_200_with_body (_content_type : Option<impl Into<HeaderValue>>, _body : impl Into<B>) -> Self {
		Self::new_with_status_and_body (consts::OK, _content_type, _body)
	}
	
	fn new_200_with_text (_body : impl Into<B>) -> Self {
		Self::new_200_with_body (Some (ContentType::Text), _body)
	}
	
	fn new_200_with_html (_body : impl Into<B>) -> Self {
		Self::new_200_with_body (Some (ContentType::Html), _body)
	}
	
	fn new_200 () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::OK, Some (ContentType::Text), "OK\n")
	}
	
	fn new_404 () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::NOT_FOUND, Some (ContentType::Text), "404\n")
	}
	
	fn new_method_not_allowed () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::METHOD_NOT_ALLOWED, Some (ContentType::Text), "method-not-allowed\n")
	}
	
	fn ok <E> (self) -> Result<Self, E> {
		Ok (self)
	}
	
	fn ok_0 (self) -> Result<Self, ServerError> {
		Ok (self)
	}
	
	fn ready <E> (self) -> future::Ready<Result<Self, E>> {
		future::ready (Ok (self))
	}
	
	fn ready_0 (self) -> future::Ready<Result<Self, ServerError>> {
		future::ready (Ok (self))
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl ResponseExtBuild<Body> for Response<Body> {
	
	fn new_with_status_and_body (_status : StatusCode, _content_type : Option<impl Into<HeaderValue>>, _body : impl Into<Body>) -> Self {
		let mut _response = Response::new (_body.into ());
		_response.set_status_200 ();
		if let Some (_content_type) = _content_type {
			_response.set_content_type (_content_type);
		}
		_response
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub struct FileResource {
	pub path : path::PathBuf,
	pub content_type : Option<ContentType>,
	pub cache : Option<Arc<RwLock<Option<Bytes>>>>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl FileResource {
	
	pub fn new (_path : impl AsRef<path::Path>, _content_type : Option<ContentType>, _should_cache : bool) -> Self {
		FileResource {
				path : _path.as_ref () .into (),
				content_type : _content_type,
				cache : if _should_cache {
						Some (Arc::new (RwLock::new (None)))
					} else {
						None
					},
			}
	}
	
	pub fn load (&self) -> ServerResult<Bytes> {
		if let Some (_cache) = self.cache.as_ref () {
			let _cache = _cache.read () .or_wrap (0x6801d05a) ?;
			if let Some (_data) = _cache.as_ref () {
				return Ok (_data.clone ());
			}
			drop (_cache);
		}
		let _data = fs::read (&self.path) .or_wrap (0x0db95839) ?;
		let _data = Bytes::from (_data);
		if let Some (_cache) = self.cache.as_ref () {
			let mut _cache = _cache.write () .or_panic (0xf103624b);
			*_cache = Some (_data.clone ());
		}
		Ok (_data)
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl Handler for FileResource {
	
	type Future = future::Ready<ServerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		match self.load () {
			Ok (_data) => {
				let mut _response = Response::new_200_with_body (self.content_type, _data);
				future::ready (Ok (_response.map (BodyWrapper::new)))
			}
			Err (_error) =>
				future::ready (Err (_error)),
		}
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub struct StaticResource {
	pub data : Bytes,
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl StaticResource {
	
	pub fn new (_data : impl Into<Bytes>, _content_type : Option<ContentType>) -> Self {
		StaticResource {
				data : _data.into (),
				content_type : _content_type,
			}
	}
	
	pub fn load_from_path (_path : impl AsRef<path::Path>, _content_type : Option<ContentType>) -> ServerResult<Self> {
		let _data = fs::read (_path) ?;
		Ok (Self::new (_data, _content_type))
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl Handler for StaticResource {
	
	type Future = future::Ready<ServerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let mut _response = Response::new_200_with_body (self.content_type, self.data.clone ());
		future::ready (Ok (_response.map (BodyWrapper::new)))
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub struct EmbeddedResource {
	pub data : &'static [u8],
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl EmbeddedResource {
	
	pub const fn new (_content_type : Option<ContentType>, _data : &'static [u8]) -> Self {
		EmbeddedResource {
				data : _data,
				content_type : _content_type,
			}
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl Handler for EmbeddedResource {
	
	type Future = future::Ready<ServerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let mut _response = Response::new_200_with_body (self.content_type, self.data.clone ());
		future::ready (Ok (_response.map (BodyWrapper::new)))
	}
}

