

use crate::prelude::*;




#[ derive (Clone) ]
#[ cfg (feature = "hss-routes") ]
pub struct Routes {
	internals : RoutesInternals,
}

#[ cfg (feature = "hss-routes") ]
struct RoutesInternals0 {
	tree : path_tree::PathTree<Arc<Route>>,
	list : Vec<Arc<Route>>,
	fallback : Option<HandlerDynArc>,
}

#[ cfg (feature = "hss-routes") ]
type RoutesInternals = Arc<RoutesInternals0>;




#[ cfg (feature = "hss-routes") ]
impl Routes {
	
	pub fn builder () -> RoutesBuilder {
		RoutesBuilder::new ()
	}
	
	pub fn into_builder (self) -> RoutesBuilder {
		let _routes = self.internals.list.clone ();
		let _fallback = self.internals.fallback.clone ();
		RoutesBuilder {
				routes : _routes,
				fallback : _fallback,
			}
	}
	
	pub fn resolve (&self, _path : &str) -> RoutesResult<Option<RouteMatched>> {
		if let Some ((_route, _parameters)) = self.internals.tree.find (_path) {
			let _route = _route.clone ();
			let _parameters = _parameters.into_iter () .map (|(_name, _value)| (String::from (_name), String::from (_value))) .collect ();
			let _matched = RouteMatched {
					routes : self.clone (),
					route : _route,
					parameters : _parameters,
				};
			Ok (Some (_matched))
		} else {
			Ok (None)
		}
	}
	
	pub fn routes (&self) -> impl Iterator<Item = &Arc<Route>> {
		self.internals.list.iter ()
	}
	
	pub fn handle (&self, _request : Request<Body>) -> HandlerFutureDynBox {
		match self.try_handle (_request) {
			Ok (_future) =>
				_future,
			Err (_request) =>
				HandlerFutureDynBox::ready_error (failed! (0x15c0a773, "no route matched for `{}`", _request.uri () .path ())),
		}
	}
	
	pub fn try_handle (&self, _request : Request<Body>) -> Result<HandlerFutureDynBox, Request<Body>> {
		let _path = _request.uri () .path ();
		let _route_matched = match self.resolve (_path) {
			Ok (_route_matched) =>
				_route_matched,
			Err (_error) =>
				return Ok (HandlerFutureDynBox::ready_error (_error.else_wrap (0xf78de1d8))),
		};
		if let Some (_route_matched) = _route_matched {
			let _route = _route_matched.route.clone ();
			Ok (_route.handle (_request, _route_matched))
		} else if let Some (_fallback) = self.internals.fallback.as_ref () {
			Ok (_fallback.delegate (_request))
		} else {
			Err (_request)
		}
	}
}


#[ cfg (feature = "hss-routes") ]
impl Route {
	
	pub fn handle (&self, _request : Request<Body>, _route_matched : RouteMatched) -> HandlerFutureDynBox {
		let mut _request = _request;
		match self.handler {
			RouteHandler::HandlerDynArc (ref _handler) => {
				_request.extensions_mut () .insert (_route_matched);
				_handler.handle (_request)
			}
			RouteHandler::RouteHandlerDynArc (ref _handler) =>
				_handler.handle (_request, _route_matched),
		}
	}
}


#[ cfg (feature = "hss-routes") ]
impl Handler for Routes {
	
	type Future = HandlerFutureDynBox;
	type ResponseBody = BodyDynBox;
	type ResponseBodyError = StdIoError;
	
	fn handle (&self, _request : Request<Body>) -> Self::Future {
		Routes::handle (self, _request)
	}
}




#[ cfg (feature = "hss-routes") ]
pub struct RoutesBuilder {
	pub routes : Vec<Arc<Route>>,
	pub fallback : Option<HandlerDynArc>,
}


#[ cfg (feature = "hss-routes") ]
impl RoutesBuilder {
	
	pub fn new () -> Self {
		Self {
				routes : Vec::new (),
				fallback : None,
			}
	}
	
	pub fn build (self) -> RoutesResult<Routes> {
		
		let _routes = self.routes;
		let mut _fallback = self.fallback;
		
		let mut _tree = path_tree::PathTree::new ();
		let mut _list = Vec::with_capacity (_routes.len ());
		
		for _route in _routes.into_iter () {
			if _route.path.is_empty () {
				if _fallback.is_some () {
					fail! (0x073a9b1a, "multiple fallback routes specified");
				}
				_fallback = match _route.handler {
					RouteHandler::HandlerDynArc (ref _handler) =>
						Some (HandlerDynArc::from_arc (_handler.clone ())),
					RouteHandler::RouteHandlerDynArc (_) =>
						fail! (0x6e5e324e, "invalid fallback route specified"),
				};
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
	pub fn with_route <'a, P, H, F, RB> (self, _paths : P, _handler : H) -> Self
			where
				P : Into<RoutePaths<'a>>,
				H : Handler<Future = F, ResponseBody = RB, ResponseBodyError = RB::Error> + Send + Sync + 'static,
				F : Future<Output = StdIoResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes> + Send + Sync + 'static,
				RB::Error : Error + Send + Sync + 'static,
	{
		let _handler : H = _handler.into ();
		self.with_route_dyn (_paths, _handler)
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_sync <'a, P, I, C, RB> (self, _paths : P, _handler : I) -> Self
			where
				P : Into<RoutePaths<'a>>,
				I : Into<HandlerFnSync<C, RB>>,
				C : Fn (Request<Body>) -> StdIoResult<Response<RB>> + Send + Sync + 'static,
				RB : BodyTrait<Data = Bytes> + Send + Sync + 'static,
				RB::Error : Error + Send + Sync + 'static,
	{
		let _handler : HandlerFnSync<C, RB> = _handler.into ();
		self.with_route_dyn (_paths, _handler)
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_async <'a, P, I, C, F, RB> (self, _paths : P, _handler : I) -> Self
			where
				P : Into<RoutePaths<'a>>,
				I : Into<HandlerFnAsync<C, F, RB>>,
				C : Fn (Request<Body>) -> F + Send + Sync + 'static,
				F : Future<Output = StdIoResult<Response<RB>>> + Send + 'static,
				RB : BodyTrait<Data = Bytes> + Send + Sync + 'static,
				RB::Error : Error + Send + Sync + 'static,
	{
		let _handler : HandlerFnAsync<C, F, RB> = _handler.into ();
		self.with_route_dyn (_paths, _handler)
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_fn_response <'a, P, C, RB> (self, _paths : P, _handler : C) -> Self
			where
				P : Into<RoutePaths<'a>>,
				C : Fn () -> Response<RB> + Send + Sync + 'static,
				RB : BodyTrait<Data = Bytes> + Send + Sync + 'static,
				RB::Error : Error + Send + Sync + 'static,
	{
		self.with_route_fn_sync (_paths, move |_request| Ok (_handler ()))
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_dyn <'a, P, H> (self, _paths : P, _handler : H) -> Self
			where
					H : HandlerDyn,
					P : Into<RoutePaths<'a>>,
	{
		let _handler : H = _handler.into ();
		let _handler = HandlerDynArc::new (_handler);
		self.with_route_arc (_paths, _handler)
	}
	
	#[ allow (single_use_lifetimes) ]
	pub fn with_route_arc <'a, P, I> (mut self, _paths : P, _handler : I) -> Self
			where
					I : Into<HandlerDynArc>,
					P : Into<RoutePaths<'a>>,
	{
		let mut _paths = _paths.into ();
		let _handler = _handler.into ();
		while let Some (_path) = _paths.next () {
			let _route = Route {
					path : String::from (_path),
					handler : RouteHandler::HandlerDynArc (_handler.clone_arc ()),
					extensions : http::Extensions::new (),
				};
			self = self.with_route_object (_route);
		}
		self
	}
	
	pub fn with_route_object (mut self, _route : Route) -> Self {
		let _route = Arc::new (_route);
		self.routes.push (_route);
		self
	}
}




#[ cfg (feature = "hss-routes") ]
pub struct Route {
	pub path : String,
	pub handler : RouteHandler,
	pub extensions : Extensions,
}




#[ derive (Clone) ]
#[ cfg (feature = "hss-routes") ]
pub enum RouteHandler {
	HandlerDynArc (Arc<dyn HandlerDyn>),
	RouteHandlerDynArc (Arc<dyn RouteHandlerDyn>),
}




#[ cfg (feature = "hss-routes") ]
pub trait RouteHandlerDyn
	where
		Self : Send + Sync + 'static,
{
	fn handle (&self, _request : Request<Body>, _route : RouteMatched) -> HandlerFutureDynBox;
}




#[ derive (Clone) ]
#[ cfg (feature = "hss-routes") ]
pub struct RouteMatched {
	pub routes : Routes,
	pub route : Arc<Route>,
	pub parameters : Vec<(String, String)>,
}


#[ cfg (feature = "hss-routes") ]
impl RouteMatched {
	
	
	pub fn parameter_nth (&self, _index : usize) -> &String {
		& (self.parameters.get (_index) .else_panic (0xe86a76a8)) .1
	}
	
	pub fn parameters_1 (&self) -> &String {
		self.parameter_nth (0)
	}
	
	pub fn parameters_2 (&self) -> (&String, &String) {
		(
			self.parameter_nth (0),
			self.parameter_nth (1),
		)
	}
	
	pub fn parameters_3 (&self) -> (&String, &String, &String) {
		(
			self.parameter_nth (0),
			self.parameter_nth (1),
			self.parameter_nth (2),
		)
	}
	
	pub fn parameters_4 (&self) -> (&String, &String, &String, &String) {
		(
			self.parameter_nth (0),
			self.parameter_nth (1),
			self.parameter_nth (2),
			self.parameter_nth (3),
		)
	}
	
	
	pub fn resolve_parameter_nth (_request : &Request<Body>, _index : usize) -> &String {
		Self::resolve_or_panic (_request) .parameter_nth (_index)
	}
	
	pub fn resolve_parameters_1 (_request : &Request<Body>) -> &String {
		Self::resolve_or_panic (_request) .parameters_1 ()
	}
	
	pub fn resolve_parameters_2 (_request : &Request<Body>) -> (&String, &String) {
		Self::resolve_or_panic (_request) .parameters_2 ()
	}
	
	pub fn resolve_parameters_3 (_request : &Request<Body>) -> (&String, &String, &String) {
		Self::resolve_or_panic (_request) .parameters_3 ()
	}
	
	pub fn resolve_parameters_4 (_request : &Request<Body>) -> (&String, &String, &String, &String) {
		Self::resolve_or_panic (_request) .parameters_4 ()
	}
	
	pub fn resolve (_request : &Request<Body>) -> Option<&Self> {
		_request.extensions () .get ()
	}
	
	fn resolve_or_panic (_request : &Request<Body>) -> &Self {
		Self::resolve (_request) .else_panic (0x4c9197b5)
	}
}




#[ cfg (feature = "hss-routes") ]
pub enum RoutePaths <'a> {
	Single (&'a str),
	Slice (&'a [&'a str]),
	Fallback,
	None,
}


#[ cfg (feature = "hss-routes") ]
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

#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a str> for RoutePaths<'a> {
	fn from (_path : &'a str) -> Self {
		RoutePaths::Single (_path)
	}
}

#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a [&'a str]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str]) -> Self {
		RoutePaths::Slice (_paths)
	}
}

#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a [&'a str; 2]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 2]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a [&'a str; 3]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 3]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a [&'a str; 4]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 4]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}
#[ cfg (feature = "hss-routes") ]
impl <'a> From<&'a [&'a str; 5]> for RoutePaths<'a> {
	fn from (_paths : &'a [&'a str; 5]) -> Self {
		RoutePaths::Slice (&_paths[..])
	}
}

