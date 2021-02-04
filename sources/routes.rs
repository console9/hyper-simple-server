

use crate::prelude::*;




pub struct Routes {
	internals : RoutesInternals,
}

struct RoutesInternals0 {
	tree : path_tree::PathTree<Arc<Route>>,
	list : Vec<Arc<Route>>,
	fallback : Option<HandlerDynArc>,
}

type RoutesInternals = Arc<RoutesInternals0>;




impl Routes {
	
	pub fn builder () -> RoutesBuilder {
		RoutesBuilder::new ()
	}
	
	pub fn resolve (&self, _path : &str) -> ServerResult<Option<RouteMatched>> {
		if let Some ((_route, _parameters)) = self.internals.tree.find (_path) {
			let _route = _route.clone ();
			let _parameters = _parameters.into_iter () .map (|(_name, _value)| (String::from (_name), String::from (_value))) .collect ();
			let _matched = RouteMatched {
					route : _route,
					parameters : _parameters
				};
			Ok (Some (_matched))
		} else {
			Ok (None)
		}
	}
	
	pub fn routes (&self) -> impl Iterator<Item = Arc<Route>> + '_ {
		self.internals.list.iter () .map (Arc::clone)
	}
}


impl Handler for Routes {
	
	type Future = Pin<Box<dyn Future<Output = ServerResult<Response<BodyDynBox>>> + Send>>;
	type ResponseBody = BodyDynBox;
	type ResponseBodyError = ServerError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		let _path = _request.uri () .path ();
		let _route_matched = match self.resolve (_path) {
			Ok (_route_matched) =>
				_route_matched,
			Err (_error) =>
				return Box::pin (future::ready (Err (_error))),
		};
		if let Some (_route_matched) = _route_matched {
			let _route = _route_matched.route.clone ();
			let mut _request = _request;
			_request.extensions_mut () .insert (_route_matched);
			_route.handler.handle (_request)
		} else if let Some (_fallback) = self.internals.fallback.as_ref () {
			_fallback.handle (_request)
		} else {
			Box::pin (future::ready (Err (error_with_format (0x15c0a773, format_args! ("no route matched path `{}`", _path)))))
		}
	}
}




pub struct RoutesBuilder {
	pub routes : Vec<Route>,
}


impl RoutesBuilder {
	
	pub fn new () -> Self {
		Self {
				routes : Vec::new (),
			}
	}
	
	pub fn build (self) -> ServerResult<Routes> {
		
		let _routes = self.routes;
		let _routes = _routes.into_iter () .map (Arc::new) .collect::<Vec<_>> ();
		
		let mut _tree = path_tree::PathTree::new ();
		let mut _list = Vec::with_capacity (_routes.len ());
		let mut _fallback = None;
		
		for _route in _routes.into_iter () {
			if _route.path.is_empty () {
				if _fallback.is_some () {
					return Err (error_with_message (0x073a9b1a, "multiple fallback routes specified"));
				}
				_fallback = Some (_route.handler.clone ());
			} else {
				_tree.insert (&_route.path, _route.clone ());
				_list.push (_route);
			}
		}
		
		let _self = RoutesInternals0 {
				tree : _tree,
				list : _list,
				fallback : _fallback,
			};
		let _self = Routes {
				internals : Arc::new (_self),
			};
		
		Ok (_self)
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route <'a, P, I, H, F, RB, RBE> (self, _paths : P, _handler : I) -> Self
			where
				P : Into<RoutePaths<'a>>,
				I : Into<H>,
				H : Handler<Future = F, ResponseBody = RB, ResponseBodyError = RBE> + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : H = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_sync <'a, P, H, C, RB, RBE> (self, _paths : P, _handler : H) -> Self
			where
				P : Into<RoutePaths<'a>>,
				H : Into<HandlerFnSync<C, RB, RBE>>,
				C : Fn (Request<Body>) -> ServerResult<Response<RB>> + Send + Sync + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnSync<C, RB, RBE> = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_async <'a, P, H, C, F, RB, RBE> (self, _paths : P, _handler : H) -> Self
			where
				P : Into<RoutePaths<'a>>,
				H : Into<HandlerFnAsync<C, F, RB, RBE>>,
				C : Fn (Request<Body>) -> F + Send + Sync + 'static,
				F : Future<Output = ServerResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes, Error = RBE> + Send + 'static,
				RBE : Error + Send + 'static,
	{
		let _handler : HandlerFnAsync<C, F, RB, RBE> = _handler.into ();
		self.with_route_dyn (_paths, _handler.into_boxed ())
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_dyn <'a, P> (mut self, _paths : P, _handler : HandlerDynArc) -> Self
			where
					P : Into<RoutePaths<'a>>
	{
		let mut _paths = _paths.into ();
		while let Some (_path) = _paths.next () {
			let _route = Route {
					path : String::from (_path),
					handler : _handler.clone (),
					debug : None,
				};
			self.routes.push (_route);
		}
		self
	}
}




pub struct Route {
	pub path : String,
	pub handler : HandlerDynArc,
	pub debug : Option<Box<dyn fmt::Debug + Send + Sync>>,
}


pub struct RouteMatched {
	pub route : Arc<Route>,
	pub parameters : Vec<(String, String)>,
}




pub enum RoutePaths <'a> {
	Single (&'a str),
	Slice (&'a [&'a str]),
	Fallback,
	None,
}


impl <'a> RoutePaths<'a> {
	
	fn next (&mut self) -> Option<&'a str> {
		match self {
			RoutePaths::Single (_path) => {
				let _path = *_path;
				*self = RoutePaths::None;
				Some (_path)
			}
			RoutePaths::Slice (_paths) => {
				if let Some ((_first, _rest)) = _paths.split_first () {
					*self = RoutePaths::Slice (_rest);
					Some (_first)
				} else {
					*self = RoutePaths::None;
					None
				}
			}
			RoutePaths::Fallback => {
				*self = RoutePaths::None;
				Some ("")
			}
			RoutePaths::None =>
				None,
		}
	}
}

impl <'a> From<&'a str> for RoutePaths<'a> {
	fn from (_path : &'a str) -> Self {
		RoutePaths::Single (_path)
	}
}

impl <'a> From<&'a [&'a str]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str]) -> Self {
		RoutePaths::Slice (_paths)
	}
}

impl <'a> From<&'a [&'a str; 2]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 2]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
impl <'a> From<&'a [&'a str; 3]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 3]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
impl <'a> From<&'a [&'a str; 4]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 4]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
impl <'a> From<&'a [&'a str; 5]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 5]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
