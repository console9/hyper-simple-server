

#![ allow (unused_import_braces) ]
#![ allow (unreachable_pub) ]




#[ cfg (feature = "http") ]
pub use ::http::{
		
		request::Request,
		request::Parts as RequestParts,
		request::Builder as RequestBuilder,
		
		response::Response,
		response::Parts as ResponseParts,
		response::Builder as ResponseBuilder,
		
		uri::Uri,
		version::Version,
		method::Method,
		status::StatusCode as Status,
		
		header::HeaderMap,
		header::HeaderName,
		header::HeaderValue,
};

#[ cfg (feature = "http-body") ]
pub use ::http_body::{
		
		Body as BodyTrait,
		SizeHint as BodySizeHint,
		Data as BodyData,
		Trailers as BodyTrailers,
	};

#[ cfg (feature = "bytes") ]
pub use ::bytes::{
		
		Bytes,
		Buf,
	};

#[ cfg (feature = "hyper") ]
pub use ::hyper::{
		
		body::Body,
	};




#[ allow (dead_code) ]
#[ cfg (feature = "http") ]
pub type Headers = HeaderMap<HeaderValue>;

