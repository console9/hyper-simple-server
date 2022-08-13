

use crate::prelude::*;




#[ cfg (feature = "hss-resources") ]
pub struct FileResource {
	pub path : path::PathBuf,
	pub content_type : Option<ContentType>,
	pub cache : Option<Arc<RwLock<Option<Bytes>>>>,
}


#[ cfg (feature = "hss-resources") ]
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
	
	pub fn load (&self) -> HandlerResult<Bytes> {
		if let Some (_cache) = self.cache.as_ref () {
			let _cache = _cache.read () .unwrap ();  // FIXME:  else_wrap
			if let Some (_data) = _cache.as_ref () {
				return Ok (_data.clone ());
			}
			mem::drop (_cache);
		}
		let _data = fs::read (&self.path) .unwrap ();  // FIXME:  else_wrap
		let _data = Bytes::from (_data);
		if let Some (_cache) = self.cache.as_ref () {
			let mut _cache = _cache.write () .unwrap ();  // FIXME:  else_wrap
			*_cache = Some (_data.clone ());
		}
		Ok (_data)
	}
	
	pub fn new_text (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Text), false)
	}
	
	pub fn new_html (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Html), false)
	}
	
	pub fn new_css (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Css), false)
	}
	
	pub fn new_js (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Js), false)
	}
	
	pub fn new_json (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Json), false)
	}
	
	pub fn new_xml (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Xml), false)
	}
	
	pub fn new_png (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Png), false)
	}
	
	pub fn new_jpeg (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Jpeg), false)
	}
	
	pub fn new_svg (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Svg), false)
	}
	
	pub fn new_icon (_path : impl AsRef<path::Path>) -> Self {
		Self::new (_path, Some (ContentType::Icon), false)
	}
	
	pub fn response (&self) -> HandlerResult<Response<Body>> {
		let _data = self.load () ?;
		let _response = Response::new_200_with_body (_data, self.content_type);
		Ok (_response)
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-resources") ]
impl Handler for FileResource {
	
	type Future = future::Ready<HandlerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = HandlerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		match self.response () {
			Ok (_response) => {
				future::ready (Ok (_response.map (BodyWrapper::new)))
			}
			Err (_error) =>
				future::ready (Err (_error)),
		}
	}
}




#[ cfg (feature = "hss-resources") ]
pub struct BytesResource {
	pub data : Bytes,
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-resources") ]
impl BytesResource {
	
	pub fn new (_data : impl Into<Bytes>, _content_type : Option<ContentType>) -> Self {
		BytesResource {
				data : _data.into (),
				content_type : _content_type,
			}
	}
	
	pub fn new_text (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Text))
	}
	
	pub fn new_html (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Html))
	}
	
	pub fn new_css (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Css))
	}
	
	pub fn new_js (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Js))
	}
	
	pub fn new_json (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Json))
	}
	
	pub fn new_xml (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Xml))
	}
	
	pub fn new_png (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Png))
	}
	
	pub fn new_jpeg (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Jpeg))
	}
	
	pub fn new_svg (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Svg))
	}
	
	pub fn new_icon (_data : impl Into<Bytes>) -> Self {
		Self::new (_data, Some (ContentType::Icon))
	}
	
	pub fn load_from_path (_path : impl AsRef<path::Path>, _content_type : Option<ContentType>) -> StdIoResult<Self> {
		let _data = fs::read (_path) ?;
		Ok (Self::new (_data, _content_type))
	}
	
	pub fn response (&self) -> Response<Body> {
		Response::new_200_with_body (self.data.clone (), self.content_type)
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-resources") ]
impl Handler for BytesResource {
	
	type Future = future::Ready<HandlerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = HandlerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let _response = self.response ();
		future::ready (Ok (_response.map (BodyWrapper::new)))
	}
}




#[ cfg (feature = "hss-resources") ]
pub struct EmbeddedResource {
	pub data : &'static [u8],
	pub content_type : Option<ContentType>,
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-resources") ]
impl EmbeddedResource {
	
	pub fn new (_data : &'static (impl AsRef<[u8]> + ?Sized), _content_type : Option<ContentType>) -> Self {
		let _data = _data.as_ref ();
		EmbeddedResource {
				data : _data,
				content_type : _content_type,
			}
	}
	
	pub const fn new_const (_data : &'static [u8], _content_type : Option<ContentType>) -> Self {
		EmbeddedResource {
				data : _data,
				content_type : _content_type,
			}
	}
	
	pub fn new_text (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Text))
	}
	
	pub fn new_html (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Html))
	}
	
	pub fn new_css (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Css))
	}
	
	pub fn new_js (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Js))
	}
	
	pub fn new_json (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Json))
	}
	
	pub fn new_xml (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Xml))
	}
	
	pub fn new_png (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Png))
	}
	
	pub fn new_jpeg (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Jpeg))
	}
	
	pub fn new_svg (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Svg))
	}
	
	pub fn new_icon (_data : &'static (impl AsRef<[u8]> + ?Sized)) -> Self {
		Self::new (_data, Some (ContentType::Icon))
	}
	
	pub fn response (&self) -> Response<Body> {
		Response::new_200_with_body (self.data, self.content_type)
	}
}


#[ cfg (feature = "hss-handler") ]
#[ cfg (feature = "hss-resources") ]
impl Handler for EmbeddedResource {
	
	type Future = future::Ready<HandlerResult<Response<Self::ResponseBody>>>;
	type ResponseBody = BodyWrapper<Body>;
	type ResponseBodyError = HandlerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let _response = self.response ();
		future::ready (Ok (_response.map (BodyWrapper::new)))
	}
}

