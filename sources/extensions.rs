

use crate::prelude::*;




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub trait RequestExt <Body : BodyTrait> {
	
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
impl <Body : BodyTrait> RequestExt<Body> for Request<Body> {
	
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
pub trait ResponseExt <Body : BodyTrait> {
	
	fn set_status (&mut self, _status : StatusCode) -> &mut Self;
	
	fn set_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	fn add_header (&mut self, _name : impl IntoHeaderName, _value : impl Into<HeaderValue>) -> &mut Self;
	
	fn set_body (&mut self, _body : impl Into<Body>) -> &mut Self;
	
	fn set_status_200 (&mut self) -> &mut Self {
		self.set_status (consts::OK)
	}
	
	fn set_content_type (&mut self, _content_type : impl Into<HeaderValue>) -> &mut Self {
		self.set_header (consts::CONTENT_TYPE, _content_type)
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl <Body : BodyTrait> ResponseExt<Body> for Response<Body> {
	
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
	
	fn set_body (&mut self, _body : impl Into<Body>) -> &mut Self {
		*self.body_mut () = _body.into ();
		self
	}
}




#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub trait ResponseExtBuild <Body : BodyTrait> : Sized {
	
	fn new_with_status_and_body (_status : StatusCode, _content_type : Option<impl Into<HeaderValue>>, _body : impl Into<Body>) -> Self;
	
	fn new_empty () -> Self where Body : Default {
		let _content_type : Option<ContentType> = None;
		Self::new_with_status_and_body (consts::OK, _content_type, Body::default ())
	}
	
	fn new_200_with_text (_body : impl Into<Body>) -> Self {
		Self::new_with_status_and_body (consts::OK, Some (ContentType::Text), _body)
	}
	
	fn new_200_with_html (_body : impl Into<Body>) -> Self {
		Self::new_with_status_and_body (consts::OK, Some (ContentType::Html), _body)
	}
	
	fn new_200 () -> Self where Body : From<&'static str> {
		Self::new_with_status_and_body (consts::OK, Some (ContentType::Text), "OK\n")
	}
	
	fn new_404 () -> Self where Body : From<&'static str> {
		Self::new_with_status_and_body (consts::NOT_FOUND, Some (ContentType::Text), "404\n")
	}
	
	fn new_method_not_allowed () -> Self where Body : From<&'static str> {
		Self::new_with_status_and_body (consts::METHOD_NOT_ALLOWED, Some (ContentType::Text), "method-not-allowed\n")
	}
	
	fn ok <Error> (self) -> Result<Self, Error> {
		Ok (self)
	}
	
	fn ready <Error> (self) -> future::Ready<Result<Self, Error>> {
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

