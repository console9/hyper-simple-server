

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
	
	fn header_str (&self, _name : impl AsHeaderName) -> Option<&str> {
		self.header (_name) .and_then (|_value| _value.to_str () .ok ())
	}
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
	
	fn set_header_str_static (&mut self, _name : impl IntoHeaderName, _value : &'static str) -> &mut Self {
		self.set_header (_name, HeaderValue::from_static (_value))
	}
	
	fn set_header_string (&mut self, _name : impl IntoHeaderName, _value : String) -> &mut Self {
		self.set_header (_name, HeaderValue::try_from (_value) .or_panic (0x627a7cff))
	}
	
	fn add_header_str_static (&mut self, _name : impl IntoHeaderName, _value : &'static str) -> &mut Self {
		self.add_header (_name, HeaderValue::from_static (_value))
	}
	
	fn add_header_string (&mut self, _name : impl IntoHeaderName, _value : String) -> &mut Self {
		self.add_header (_name, HeaderValue::try_from (_value) .or_panic (0x1ac00c46))
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
		Self : ResponseExt<B>,
{
	fn new_with_status_and_body (_status : StatusCode, _body : impl Into<B>, _content_type : Option<impl Into<HeaderValue>>) -> Self;
	
	fn new_empty () -> Self where B : Default {
		let _content_type : Option<ContentType> = None;
		Self::new_with_status_and_body (consts::OK, B::default (), _content_type)
	}
	
	fn new_with_status (_status : StatusCode) -> Self where B : Default {
		let _content_type : Option<ContentType> = None;
		Self::new_with_status_and_body (_status, B::default (), _content_type)
	}
	
	fn new_200_with_body (_body : impl Into<B>, _content_type : Option<impl Into<HeaderValue>>) -> Self {
		Self::new_with_status_and_body (consts::OK, _body, _content_type)
	}
	
	fn new_200_with_text (_body : impl Into<B>) -> Self {
		Self::new_200_with_body (_body, Some (ContentType::Text))
	}
	
	fn new_200_with_html (_body : impl Into<B>) -> Self {
		Self::new_200_with_body (_body, Some (ContentType::Html))
	}
	
	fn new_200 () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::OK, "OK\n", Some (ContentType::Text))
	}
	
	fn new_301 (_location : impl Into<HeaderValue>) -> Self where B : Default {
		Self::new_redirect_with_code (consts::MOVED_PERMANENTLY, _location)
	}
	
	fn new_302 (_location : impl Into<HeaderValue>) -> Self where B : Default {
		Self::new_redirect_with_code (consts::FOUND, _location)
	}
	
	fn new_303 (_location : impl Into<HeaderValue>) -> Self where B : Default {
		Self::new_redirect_with_code (consts::SEE_OTHER, _location)
	}
	
	fn new_307 (_location : impl Into<HeaderValue>) -> Self where B : Default {
		Self::new_redirect_with_code (consts::TEMPORARY_REDIRECT, _location)
	}
	
	fn new_308 (_location : impl Into<HeaderValue>) -> Self where B : Default {
		Self::new_redirect_with_code (consts::PERMANENT_REDIRECT, _location)
	}
	
	fn new_301_str_static (_location : &'static str) -> Self where B : Default {
		Self::new_301 (HeaderValue::from_static (_location))
	}
	
	fn new_302_str_static (_location : &'static str) -> Self where B : Default {
		Self::new_302 (HeaderValue::from_static (_location))
	}
	
	fn new_303_str_static (_location : &'static str) -> Self where B : Default {
		Self::new_303 (HeaderValue::from_static (_location))
	}
	
	fn new_307_str_static (_location : &'static str) -> Self where B : Default {
		Self::new_307 (HeaderValue::from_static (_location))
	}
	
	fn new_308_str_static (_location : &'static str) -> Self where B : Default {
		Self::new_308 (HeaderValue::from_static (_location))
	}
	
	fn new_301_string (_location : String) -> Self where B : Default {
		Self::new_301 (HeaderValue::try_from (_location) .or_panic (0x15f9b93c))
	}
	
	fn new_302_string (_location : String) -> Self where B : Default {
		Self::new_302 (HeaderValue::try_from (_location) .or_panic (0x0a94e02d))
	}
	
	fn new_303_string (_location : String) -> Self where B : Default {
		Self::new_303 (HeaderValue::try_from (_location) .or_panic (0xa87f95a7))
	}
	
	fn new_307_string (_location : String) -> Self where B : Default {
		Self::new_307 (HeaderValue::try_from (_location) .or_panic (0xd0e3f0f9))
	}
	
	fn new_308_string (_location : String) -> Self where B : Default {
		Self::new_308 (HeaderValue::try_from (_location) .or_panic (0x69839071))
	}
	
	fn new_redirect_with_code (_status : StatusCode, _location : impl Into<HeaderValue>) -> Self where B : Default {
		let mut _response = Self::new_with_status (_status);
		_response.set_header (consts::LOCATION, _location);
		_response
	}
	
	fn new_404 () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::NOT_FOUND, "404\n", Some (ContentType::Text))
	}
	
	fn new_method_not_allowed () -> Self where B : From<&'static str> {
		Self::new_with_status_and_body (consts::METHOD_NOT_ALLOWED, "method-not-allowed\n", Some (ContentType::Text))
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
	
	fn new_with_status_and_body (_status : StatusCode, _body : impl Into<Body>, _content_type : Option<impl Into<HeaderValue>>) -> Self {
		let mut _response = Response::new (_body.into ());
		_response.set_status (_status);
		if let Some (_content_type) = _content_type {
			_response.set_content_type (_content_type);
		}
		_response
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct FileResource {
	pub path : path::PathBuf,
	pub content_type : Option<ContentType>,
	pub cache : Option<Arc<RwLock<Option<Bytes>>>>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-handler") ]
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
#[ cfg (feature = "hss-handler") ]
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
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct StaticResource {
	pub data : Bytes,
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-handler") ]
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
#[ cfg (feature = "hss-handler") ]
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
#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-extensions") ]
pub struct EmbeddedResource {
	pub data : &'static [u8],
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-handler") ]
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
#[ cfg (feature = "hss-handler") ]
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

