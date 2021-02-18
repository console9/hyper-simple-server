

#![ allow (unused_import_braces) ]




use crate::prelude::*;




#[ cfg (feature = "http") ]
pub use http::{
		
		request::Request,
		request::Parts as RequestParts,
		request::Builder as RequestBuilder,
		
		response::Response,
		response::Parts as ResponseParts,
		response::Builder as ResponseBuilder,
		
		uri::Uri,
		version::Version,
		method::Method,
		status::StatusCode,
		
		header::HeaderMap,
		header::HeaderName,
		header::HeaderValue,
		
		header::AsHeaderName,
		header::IntoHeaderName,
};

#[ cfg (feature = "http-body") ]
pub use http_body::{
		
		Body as BodyTrait,
		SizeHint as BodySizeHint,
		Data as BodyData,
		Trailers as BodyTrailers,
	};

#[ cfg (feature = "bytes") ]
pub use bytes::{
		
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




#[ derive (Copy, Clone) ]
#[ allow (dead_code) ]
#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub enum ContentType {
	
	// https://docs.rs/headers/0.3.3/headers/struct.ContentType.html
	// https://docs.rs/mime/0.3.16/mime/#constants
	
	Text,
	Html,
	Css,
	Js,
	
	Json,
	Xml,
	
	Png,
	Jpeg,
	Svg,
	Icon,
	
	FontTtf,
	FontOtf,
	FontWoff,
	FontWoff2,
	
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl ContentType {
	
	pub fn to_str (&self) -> &'static str {
		match self {
			
			ContentType::Text => "text/plain; charset=utf-8",
			ContentType::Html => "text/html; charset=utf-8",
			ContentType::Css => "text/css; charset=utf-8",
			ContentType::Js => "application/javascript; charset=utf-8",
			
			ContentType::Json => "application/json; charset=utf-8",
			ContentType::Xml => "application/xml; charset=utf-8",
			
			ContentType::Png => "image/png",
			ContentType::Jpeg => "image/jpeg",
			ContentType::Svg => "image/svg+xml",
			ContentType::Icon => "image/vnd.microsoft.icon",
			
			ContentType::FontTtf => "font/ttf",
			ContentType::FontOtf => "font/otf",
			ContentType::FontWoff => "font/woff",
			ContentType::FontWoff2 => "font/woff2",
		}
	}
}


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
impl Into<HeaderValue> for ContentType {
	fn into (self) -> HeaderValue {
		#[ allow (unsafe_code) ]
		unsafe {
			HeaderValue::from_maybe_shared_unchecked (Bytes::from_static (self.to_str () .as_bytes ()))
		}
	}
}




#[ allow (dead_code) ]
#[ allow (clippy::declare_interior_mutable_const) ]
#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-extensions") ]
pub mod consts {
	
	
	macro_rules! def_const {
		( $_type : ty => $_from : ident => $( $_id : ident ),+ $(,)? ) => {
			$(
				pub const $_id : $_type = ::http::$_from::$_id;
			)+
		};
		( $_type : ty => $( $_id : ident ),+ $(,)? ) => {
			$(
				pub const $_id : $_type = <$_type>::$_id;
			)+
		};
	}
	
	
	def_const! (super::Version => HTTP_09, HTTP_10, HTTP_11, HTTP_2, HTTP_3);
	def_const! (super::Method => GET, POST, PUT, DELETE, HEAD, OPTIONS, CONNECT, PATCH, TRACE);
	
	
	// NOTE:  https://docs.rs/http/0.2.3/src/http/status.rs.html#323-515
	// x-selection co | sed -r -e '\#^\s*$#d' -e 's#^\s+##' -e '\#^//#d' -e 's#\([0-9]+,\s*([^,]+).*$#\1#' -e 's#^#\t\t\t#' -e 's#$#,#' | LC_ALL=C sort | x-selection pi
	def_const! (super::StatusCode =>
			ACCEPTED,
			ALREADY_REPORTED,
			BAD_GATEWAY,
			BAD_REQUEST,
			CONFLICT,
			CONTINUE,
			CREATED,
			EXPECTATION_FAILED,
			FAILED_DEPENDENCY,
			FORBIDDEN,
			FOUND,
			GATEWAY_TIMEOUT,
			GONE,
			HTTP_VERSION_NOT_SUPPORTED,
			IM_A_TEAPOT,
			IM_USED,
			INSUFFICIENT_STORAGE,
			INTERNAL_SERVER_ERROR,
			LENGTH_REQUIRED,
			LOCKED,
			LOOP_DETECTED,
			METHOD_NOT_ALLOWED,
			MISDIRECTED_REQUEST,
			MOVED_PERMANENTLY,
			MULTIPLE_CHOICES,
			MULTI_STATUS,
			NETWORK_AUTHENTICATION_REQUIRED,
			NON_AUTHORITATIVE_INFORMATION,
			NOT_ACCEPTABLE,
			NOT_EXTENDED,
			NOT_FOUND,
			NOT_IMPLEMENTED,
			NOT_MODIFIED,
			NO_CONTENT,
			OK,
			PARTIAL_CONTENT,
			PAYLOAD_TOO_LARGE,
			PAYMENT_REQUIRED,
			PERMANENT_REDIRECT,
			PRECONDITION_FAILED,
			PRECONDITION_REQUIRED,
			PROCESSING,
			PROXY_AUTHENTICATION_REQUIRED,
			RANGE_NOT_SATISFIABLE,
			REQUEST_HEADER_FIELDS_TOO_LARGE,
			REQUEST_TIMEOUT,
			RESET_CONTENT,
			SEE_OTHER,
			SERVICE_UNAVAILABLE,
			SWITCHING_PROTOCOLS,
			TEMPORARY_REDIRECT,
			TOO_MANY_REQUESTS,
			UNAUTHORIZED,
			UNAVAILABLE_FOR_LEGAL_REASONS,
			UNPROCESSABLE_ENTITY,
			UNSUPPORTED_MEDIA_TYPE,
			UPGRADE_REQUIRED,
			URI_TOO_LONG,
			USE_PROXY,
			VARIANT_ALSO_NEGOTIATES,
		);
	
	
	// https://docs.rs/http/0.2.3/src/http/header/name.rs.html#146-965
	// x-selection co | sed -r -e '\#^\s*$#d' -e 's#^\s+##' -e '\#^//#d' -e 's#\([a-zA-Z0-9]+,\s*([^,]+).*$#\1#' -e 's#^#\t\t\t#' -e 's#$#,#' | LC_ALL=C sort | x-selection pi
	def_const! (super::HeaderName => header =>
			ACCEPT,
			ACCEPT_CHARSET,
			ACCEPT_ENCODING,
			ACCEPT_LANGUAGE,
			ACCEPT_RANGES,
			ACCESS_CONTROL_ALLOW_CREDENTIALS,
			ACCESS_CONTROL_ALLOW_HEADERS,
			ACCESS_CONTROL_ALLOW_METHODS,
			ACCESS_CONTROL_ALLOW_ORIGIN,
			ACCESS_CONTROL_EXPOSE_HEADERS,
			ACCESS_CONTROL_MAX_AGE,
			ACCESS_CONTROL_REQUEST_HEADERS,
			ACCESS_CONTROL_REQUEST_METHOD,
			AGE,
			ALLOW,
			ALT_SVC,
			AUTHORIZATION,
			CACHE_CONTROL,
			CONNECTION,
			CONTENT_DISPOSITION,
			CONTENT_ENCODING,
			CONTENT_LANGUAGE,
			CONTENT_LENGTH,
			CONTENT_LOCATION,
			CONTENT_RANGE,
			CONTENT_SECURITY_POLICY,
			CONTENT_SECURITY_POLICY_REPORT_ONLY,
			CONTENT_TYPE,
			COOKIE,
			DATE,
			DNT,
			ETAG,
			EXPECT,
			EXPIRES,
			FORWARDED,
			FROM,
			HOST,
			IF_MATCH,
			IF_MODIFIED_SINCE,
			IF_NONE_MATCH,
			IF_RANGE,
			IF_UNMODIFIED_SINCE,
			LAST_MODIFIED,
			LINK,
			LOCATION,
			MAX_FORWARDS,
			ORIGIN,
			PRAGMA,
			PROXY_AUTHENTICATE,
			PROXY_AUTHORIZATION,
			PUBLIC_KEY_PINS,
			PUBLIC_KEY_PINS_REPORT_ONLY,
			RANGE,
			REFERER,
			REFERRER_POLICY,
			REFRESH,
			RETRY_AFTER,
			SEC_WEBSOCKET_ACCEPT,
			SEC_WEBSOCKET_EXTENSIONS,
			SEC_WEBSOCKET_KEY,
			SEC_WEBSOCKET_PROTOCOL,
			SEC_WEBSOCKET_VERSION,
			SERVER,
			SET_COOKIE,
			STRICT_TRANSPORT_SECURITY,
			TE,
			TRAILER,
			TRANSFER_ENCODING,
			UPGRADE,
			UPGRADE_INSECURE_REQUESTS,
			USER_AGENT,
			VARY,
			VIA,
			WARNING,
			WWW_AUTHENTICATE,
			X_CONTENT_TYPE_OPTIONS,
			X_DNS_PREFETCH_CONTROL,
			X_FRAME_OPTIONS,
			X_XSS_PROTECTION,
		);
}

