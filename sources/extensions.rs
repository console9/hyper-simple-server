

use crate::prelude::*;




#[ cfg (feature = "hss-extensions") ]
pub trait HeadersExt
	where
		Self : Sized,
{
	fn get_header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue>;
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	
	fn get_headers (&self) -> &HeaderMap;
	fn set_headers (&mut self, _headers : HeaderMap) -> &mut Self;
	
	// FIXME:  Duplicates `get_header` and related.
	fn header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue> {
		self.get_header (_name)
	}
	
	// FIXME:  Duplicates `get_headers` and related.
	fn header_all (&self, _name : impl AsHeaderName) -> http::header::GetAll<'_, HeaderValue> {
		self.get_headers () .get_all (_name)
	}
	
	fn header_str (&self, _name : impl AsHeaderName) -> Option<&str> {
		self.get_header (_name) .and_then (|_value| _value.to_str () .ok ())
	}
	
	fn header_host (&self) -> Option<&str> {
		self.header_str (consts::HOST)
	}
	
	fn header_host_normalized (&self) -> Option<Cow<str>> {
		// FIXME:  Check if header is actually properly formatted!
		let _host = self.header_host () ?;
		let _host = _host.trim_matches ('.');
		let _is_lowercase = _host.find (char::is_uppercase) .is_none ();
		if _is_lowercase {
			Some (Cow::Borrowed (_host))
		} else {
			Some (Cow::Owned (_host.to_lowercase ()))
		}
	}
}


#[ cfg (feature = "hss-extensions") ]
impl HeadersExt for HeaderMap {
	
	fn get_header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue> {
		self.get (_name)
	}
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.insert (_name, _value.into ());
		self
	}
	
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.append (_name, _value.into ());
		self
	}
	
	fn get_headers (&self) -> &HeaderMap {
		self
	}
	
	fn set_headers (&mut self, _headers : HeaderMap) -> &mut Self {
		*self = _headers;
		self
	}
}




#[ cfg (feature = "hss-extensions") ]
pub trait RequestExt <B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
		Self : HeadersExt,
		Self : Sized,
{
	fn from_parts (_parts : RequestParts, _body : B) -> Self;
	fn into_parts (self) -> (RequestParts, B);
	
	fn is_get (&self) -> bool;
	fn is_head (&self) -> bool;
	fn is_put (&self) -> bool;
	fn is_post (&self) -> bool;
	fn is_delete (&self) -> bool;
	fn is_patch (&self) -> bool;
	fn is_options (&self) -> bool;
	fn method_str (&self) -> &str;
	
	fn uri_path (&self) -> &str;
	fn uri_query (&self) -> Option<&str>;
	
	fn body_size (&self) -> Option<u64>;
	
	fn into_body_dyn_box (self) -> Request<BodyDynBox> {
		let (_parts, _body) = self.into_parts ();
		let _body = BodyDynBox::new (_body);
		Request::from_parts (_parts, _body)
	}
}


#[ cfg (feature = "hss-extensions") ]
impl <B> HeadersExt for Request<B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	fn get_header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue> {
		self.headers () .get_header (_name)
	}
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .set_header (_name, _value);
		self
	}
	
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .add_header (_name, _value);
		self
	}
	
	fn get_headers (&self) -> &HeaderMap {
		self.headers () .get_headers ()
	}
	
	fn set_headers (&mut self, _headers : HeaderMap) -> &mut Self {
		self.headers_mut () .set_headers (_headers);
		self
	}
}


#[ cfg (feature = "hss-extensions") ]
impl <B> RequestExt<B> for Request<B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
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
	fn method_str (&self) -> &str {
		self.method () .as_str ()
	}
	
	fn uri_path (&self) -> &str {
		self.uri () .path ()
	}
	
	fn uri_query (&self) -> Option<&str> {
		self.uri () .query ()
	}
	
	fn body_size (&self) -> Option<u64> {
		self.body () .size_hint () .exact ()
	}
	
	fn into_parts (self) -> (RequestParts, B) {
		Request::into_parts (self)
	}
	
	fn from_parts (_parts : RequestParts, _body : B) -> Self {
		Request::from_parts (_parts, _body)
	}
}




#[ cfg (feature = "hss-extensions") ]
pub trait ResponseExt <B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
		Self : HeadersExt,
		Self : Sized,
{
	fn from_parts (_parts : ResponseParts, _body : B) -> Self;
	fn into_parts (self) -> (ResponseParts, B);
	
	fn status (&self) -> StatusCode;
	fn set_status (&mut self, _status : StatusCode) -> &mut Self;
	
	fn status_u16 (&self) -> u16 {
		self.status () .as_u16 ()
	}
	fn set_status_u16 (&mut self, _status : u16) -> Result<&mut Self, ::http::status::InvalidStatusCode> {
		let _status = StatusCode::from_u16 (_status) ?;
		Ok (self.set_status (_status))
	}
	
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
		self.set_header (_name, HeaderValue::try_from (_value) .infallible (0x627a7cff))
	}
	
	fn add_header_str_static (&mut self, _name : impl IntoHeaderName, _value : &'static str) -> &mut Self {
		self.add_header (_name, HeaderValue::from_static (_value))
	}
	
	fn add_header_string (&mut self, _name : impl IntoHeaderName, _value : String) -> &mut Self {
		self.add_header (_name, HeaderValue::try_from (_value) .infallible (0x1ac00c46))
	}
	
	fn content_type (&self) -> Option<ContentType> {
		if let Some (_content_type) = self.get_header (consts::CONTENT_TYPE) {
			if let Ok (_content_type) = _content_type.to_str () {
				ContentType::from_str (_content_type)
			} else {
				None
			}
		} else {
			None
		}
	}
	
	fn content_type_or_unknown (&self) -> ContentType {
		self.content_type () .unwrap_or (ContentType::Unknown)
	}
	
	fn body_size (&self) -> Option<u64>;
	
	fn into_body_dyn_box (self) -> Response<BodyDynBox> {
		let (_parts, _body) = self.into_parts ();
		let _body = BodyDynBox::new (_body);
		Response::from_parts (_parts, _body)
	}
}


#[ cfg (feature = "hss-extensions") ]
impl <B> HeadersExt for Response<B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	fn get_header (&self, _name : impl AsHeaderName) -> Option<&HeaderValue> {
		self.headers () .get_header (_name)
	}
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .set_header (_name, _value);
		self
	}
	
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self {
		self.headers_mut () .add_header (_name, _value);
		self
	}
	
	fn get_headers (&self) -> &HeaderMap {
		self.headers () .get_headers ()
	}
	
	fn set_headers (&mut self, _headers : HeaderMap) -> &mut Self {
		self.headers_mut () .set_headers (_headers);
		self
	}
}


#[ cfg (feature = "hss-extensions") ]
impl <B> ResponseExt<B> for Response<B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	fn status (&self) -> StatusCode {
		self.status ()
	}
	
	fn set_status (&mut self, _status : StatusCode) -> &mut Self {
		*self.status_mut () = _status;
		self
	}
	
	fn set_body (&mut self, _body : impl Into<B>) -> &mut Self {
		*self.body_mut () = _body.into ();
		self
	}
	
	fn body_size (&self) -> Option<u64> {
		self.body () .size_hint () .exact ()
	}
	
	fn into_parts (self) -> (ResponseParts, B) {
		Response::into_parts (self)
	}
	
	fn from_parts (_parts : ResponseParts, _body : B) -> Self {
		Response::from_parts (_parts, _body)
	}
}




#[ cfg (feature = "hss-extensions") ]
pub trait ResponseExtBuild <B>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
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
	
	fn new_201 () -> Self where B : Default {
		Self::new_with_status (consts::CREATED)
	}
	
	fn new_204 () -> Self where B : Default {
		Self::new_with_status (consts::NO_CONTENT)
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
		Self::new_301 (HeaderValue::try_from (_location) .infallible (0x15f9b93c))
	}
	
	fn new_302_string (_location : String) -> Self where B : Default {
		Self::new_302 (HeaderValue::try_from (_location) .infallible (0x0a94e02d))
	}
	
	fn new_303_string (_location : String) -> Self where B : Default {
		Self::new_303 (HeaderValue::try_from (_location) .infallible (0xa87f95a7))
	}
	
	fn new_307_string (_location : String) -> Self where B : Default {
		Self::new_307 (HeaderValue::try_from (_location) .infallible (0xd0e3f0f9))
	}
	
	fn new_308_string (_location : String) -> Self where B : Default {
		Self::new_308 (HeaderValue::try_from (_location) .infallible (0x69839071))
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
	
	fn ready <E> (self) -> future::Ready<Result<Self, E>> {
		future::ready (Ok (self))
	}
}


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








#[ cfg (feature = "hss-extensions") ]
#[ cfg (feature = "tokio--rt") ]
pub trait BodyExt {
	
	fn consume_into_vec (&mut self, _buffer : &mut Vec<u8>, _limit : Option<usize>, _runtime : Option<&Runtime>) -> BodyResult;
	
	fn consume_to_vec (&mut self, _limit : Option<usize>, _runtime : Option<&Runtime>) -> BodyResult<Vec<u8>> {
		let mut _buffer = Vec::new ();
		self.consume_into_vec (&mut _buffer, _limit, _runtime) ?;
		Ok (_buffer)
	}
}


#[ cfg (feature = "hss-extensions") ]
#[ cfg (feature = "tokio--rt") ]
impl <B> BodyExt for B
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	fn consume_into_vec (&mut self, _buffer : &mut Vec<u8>, _limit : Option<usize>, _runtime : Option<&Runtime>) -> BodyResult {
		if let Some (_runtime) = _runtime {
			_runtime.block_on (body_consume_into_vec (self, _buffer, _limit))
		} else if let Ok (_runtime_handle) = tokio::RuntimeHandle::try_current () {
			_runtime_handle.block_on (body_consume_into_vec (self, _buffer, _limit))
		} else {
			fail! (0x395326d7);
		}
	}
}




#[ cfg (feature = "hss-extensions") ]
#[ cfg (feature = "tokio--rt") ]
pub async fn body_consume_into_vec <B> (_body : &mut B, _buffer : &mut Vec<u8>, _limit : Option<usize>) -> BodyResult
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	let _body_size_hint = BodyTrait::size_hint (_body);
	let _body_size_lower = _body_size_hint.lower () .try_into () .else_wrap (0x2e5e16d2) ?;
	let _limit = _limit.unwrap_or (u32::MAX as usize);
	
	if _body_size_lower > _limit {
		fail! (0x4b61ba4e);
	}
	if _body_size_lower >= 0 {
		_buffer.reserve (_body_size_lower);
	}
	
	let mut _amount = 0;
	
	loop {
		let _next = _body.data () .await;
		let _next = if let Some (_next) = _next {
			_next
		} else {
			break;
		};
		let mut _data = _next.else_wrap (0xe7956afe) ?;
		while _data.remaining () > 0 {
			let _chunk = _data.chunk ();
			let _chunk_size = _chunk.len ();
			if (_amount + _chunk_size) > _limit {
				fail! (0xf0805723);
			}
			_buffer.extend (_chunk);
			_data.advance (_chunk_size);
			_amount += _chunk_size;
		}
	}
	
	Ok (())
}


#[ cfg (feature = "hss-extensions") ]
#[ cfg (feature = "tokio--rt") ]
pub async fn body_consume_to_vec <B> (_body : &mut B, _limit : Option<usize>) -> BodyResult<Vec<u8>>
	where
		B : BodyTrait<Data = Bytes> + Send + Sync + 'static + Unpin,
		B::Error : StdError + Send + Sync + 'static,
{
	let mut _buffer = Vec::new ();
	body_consume_into_vec (_body, &mut _buffer, _limit) .await ?;
	Ok (_buffer)
}

