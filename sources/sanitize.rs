

use crate::prelude::*;


#[ cfg (feature = "hss-http") ]
#[ cfg (feature = "hss-sanitize") ]
use http::uri::{
		Uri,
		Scheme,
		Authority,
		PathAndQuery,
	};




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_scheme (_scheme : &Scheme) -> ServerResult<Option<Scheme>> {
	
	// NOTE:  The `_uri.scheme` is always safe, according to the `http::uri::scheme::Scheme2::parse_exact` implementation.
	
	Ok (None)
}




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_authority (_authority : &Authority) -> ServerResult<Option<Authority>> {
	
	let _authority = _authority.as_str ();
	
	if let Some (_offset) = _authority.find ('@') {
		
		// NOTE:  The rest of `_uri.authority` is always safe, according to the `http::uri::authority::Authority::parse` implementation.
		let _authority = Authority::try_from (&_authority[_offset + 1 ..]) .or_wrap (0xda6f459d) ?;
		
		Ok (Some (_authority))
		
	} else {
		Ok (None)
	}
}




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_path (_path : &str) -> ServerResult<Option<String>> {
	
	let _path_as_str = _path;
	let _path_as_bytes = _path_as_str.as_bytes ();
	mem::drop (_path);
	
	match _path_as_bytes {
		
		b"" =>
			// NOTE:  If the buffer is empty, it will always return `/` (and we can't detect this case), thus this branch is just a safety-net.
			Ok (Some (String::from ("/"))),
		b"/" | b"*" =>
			Ok (None),
		
		_ if _path_as_bytes[0] != b'/' =>
			return Err (error_with_code (0x0705e550)),
		
		_ => {
			
			let mut _normalize = false;
			if !_normalize && _path_as_str.contains ("//") {
				_normalize = true;
			}
			if !_normalize && (_path_as_str.contains ("/./") || _path_as_bytes.ends_with (b"/.")) {
				_normalize = true;
			}
			if !_normalize && (_path_as_str.contains ("/../") || _path_as_bytes.ends_with (b"/..")) {
				_normalize = true;
			}
			if !_normalize && _path_as_bytes.contains (&b'%') {
				_normalize = true;
			}
			
			if _normalize {
				
				let mut _buffer = Vec::with_capacity (_path_as_bytes.len ());
				enum State {
					Normal,
					Percent0,
					Percent1 (u8),
				}
				let mut _rest = _path_as_bytes;
				let mut _last_push = None;
				let mut _last_state = State::Normal;
				
				loop {
					
					let _current = if let [_head, _tail @ ..] = _rest {
						_rest = _tail;
						*_head
					} else {
						break;
					};
					
					let (mut _next_push, _next_state) = match _last_state {
						State::Normal =>
							// NOTE:  Based on `http::uri::PathAndQuery::from_shared` implementation.
							match _current {
								// NOTE:  These two cases should always be percent-encoded, thus this branch is just a safety-net.
								b'?' | b'#' =>
									(Some (_current), State::Normal),
								b'%' =>
									(None, State::Percent0),
								0x21 | 0x24 ..= 0x3B | 0x3D | 0x40 ..= 0x5F | 0x61 ..= 0x7A | 0x7C | 0x7E =>
									(Some (_current), State::Normal),
								_ =>
									return Err (error_with_code (0x9c6c8644)),
							},
						State::Percent0 | State::Percent1 (_) => {
							let _digit_2 = match _current {
								b'0' ..= b'9' => _current - b'0',
								b'A' ..= b'F' => _current - b'A' + 10,
								b'a' ..= b'f' => _current - b'a' + 10,
								_ =>
									return Err (error_with_code (0x563f825c)),
							};
							match _last_state {
								State::Percent0 =>
									(None, State::Percent1 (_digit_2)),
								State::Percent1 (_digit_1) => {
									let _byte = (_digit_1 << 4) | _digit_2;
									(Some (_byte), State::Normal)
								}
								_ =>
									panic_with_code (0xacb15742),
							}
						}
					};
					
					if (_next_push == Some (b'/')) && (_last_push == Some (b'/')) {
						_next_push = None;
					}
					
					if let Some (_next_push) = _next_push {
						let _percent = match _next_push {
							b'?' | b'#' | b'%' =>
								true,
							0x21 | 0x24 ..= 0x3B | 0x3D | 0x40 ..= 0x5F | 0x61 ..= 0x7A | 0x7C | 0x7E =>
								false,
							_ =>
								true,
						};
						if _percent {
							let _digit_1 = _next_push >> 4;
							let _digit_2 = _next_push & 0x0f;
							_buffer.push (b'%');
							_buffer.push (if _digit_1 <= 9 { _digit_1 + b'0' } else { _digit_1 - 10 + b'A' });
							_buffer.push (if _digit_2 <= 9 { _digit_2 + b'0' } else { _digit_2 - 10 + b'A' });
						} else {
							_buffer.push (_next_push);
						}
					}
					
					_last_push = _next_push;
					_last_state = _next_state;
				}
				
				match _last_state {
					State::Normal => (),
					State::Percent0 | State::Percent1 (_) =>
						return Err (error_with_code (0x574c1224)),
				}
				
				let mut _buffer = String::from_utf8 (_buffer) .or_wrap (0x88642ea3) ?;
				
				while let Some (_offset) = _buffer.rfind ("//") {
					_buffer.remove (_offset);
				}
				if _buffer.ends_with ("/.") {
					_buffer.push ('/');
				}
				while let Some (_offset) = _buffer.rfind ("/./") {
					_buffer.replace_range (_offset .. _offset + 3, "/");
				}
				if _buffer.ends_with ("/..") {
					_buffer.push ('/');
				}
				while let Some (_offset_2) = _buffer.find ("/../") {
					let _offset_1 = if _offset_2 > 0 { _buffer[0.._offset_2].rfind ('/') .or_panic (0x3693e145) } else { 0 };
					_buffer.replace_range (_offset_1 .. _offset_2 + 4, "/");
				}
				
				Ok (Some (_buffer))
				
			} else {
				Ok (None)
			}
		}
	}
}




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_query (_query : &str) -> ServerResult<Option<String>> {
	
	match _query {
		
		"" =>
			Ok (Some (String::new ())),
		
		_ =>
			// FIXME:  For the moment we don't validate queries!
			Ok (None),
	}
}




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_path_and_query (_path_and_query : &PathAndQuery) -> ServerResult<Option<PathAndQuery>> {
	
	let _path = sanitize_path (_path_and_query.path ()) ?;
	
	let _query = if let Some (_query) = _path_and_query.query () {
		sanitize_query (_query) ?
	} else {
		None
	};
	
	if _path.is_none () && _query.is_none () {
		return Ok (None);
	}
	
	let _path = _path.as_ref () .map (String::as_str) .unwrap_or_else (|| _path_and_query.path ());
	let _query = _query.as_ref () .map (String::as_str) .or_else (|| _path_and_query.query ()) .unwrap_or ("");
	
	let mut _buffer = String::with_capacity (_path.len () + _query.len () + 1);
	_buffer.push_str (_path);
	if ! _query.is_empty () {
		_buffer.push ('?');
		_buffer.push_str (_query);
	}
	
	let _path_and_query = PathAndQuery::try_from (_buffer) .or_wrap (0x7d1433ad) ?;
	
	Ok (Some (_path_and_query))
}




#[ cfg (feature = "hss-sanitize") ]
pub fn sanitize_uri (_uri : &Uri) -> ServerResult<Option<Uri>> {
	
//	eprintln! ("[dd] [34b470a9]  `{}` | scheme: {:?} | authority: {:?} | path: {:?} | query: {:?}", _uri, _uri.scheme (), _uri.authority (), _uri.path (), _uri.query ());
	
	let _scheme = if let Some (_scheme) = _uri.scheme () {
		sanitize_scheme (_scheme) ?
	} else {
		None
	};
	
	let _authority = if let Some (_authority) = _uri.authority () {
		sanitize_authority (_authority) ?
	} else {
		None
	};
	
	let _path_and_query = if let Some (_path_and_query) = _uri.path_and_query () {
		sanitize_path_and_query (_path_and_query) ?
	} else {
		// NOTE:  This seems to be a corner-case:  if one submits just `host`, one gets an authority, but not a path!
		Some (PathAndQuery::from_static ("/"))
	};
	
	if _scheme.is_none () && _authority.is_none () && _path_and_query.is_none () {
		return Ok (None);
	}
	
	// NOTE:  `http::uri::Uri::from_parts` fails if there is `authority` but not `scheme`, although this is what we get if one submits just `host`!
	
	let _uri_parts = _uri.clone ();
	
	#[ allow (unsafe_code) ]
	let mut _uri_parts : (Scheme, Authority, PathAndQuery) = unsafe { mem::transmute (_uri_parts) };
	
	if let Some (_scheme) = _scheme {
		_uri_parts.0 = _scheme;
	}
	if let Some (_authority) = _authority {
		_uri_parts.1 = _authority;
	}
	if let Some (_path_and_query) = _path_and_query {
		_uri_parts.2 = _path_and_query;
	}
	
	#[ allow (unsafe_code) ]
	let _uri : Uri = unsafe { mem::transmute (_uri_parts) };
	
	return Ok (Some (_uri));
}

