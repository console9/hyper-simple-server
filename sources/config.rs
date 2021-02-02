

use crate::prelude::*;




#[ derive (Clone, Debug) ]
pub struct Configuration {
	pub endpoint : Endpoint,
}




#[ derive (Clone, Debug) ]
pub struct Endpoint {
	pub address : EndpointAddress,
	pub protocol : EndpointProtocol,
}


#[ derive (Clone, Debug) ]
pub enum EndpointAddress {
	Socket (net::SocketAddr),
	Descriptor (u32),
}


#[ derive (Copy, Clone, Debug) ]
pub enum EndpointProtocol {
	Http1,
	Http2,
	Http12,
}




impl Default for Configuration {
	
	fn default () -> Self {
		Configuration {
				endpoint : Endpoint::default (),
			}
	}
}


impl Default for Endpoint {
	
	fn default () -> Self {
		Endpoint {
				address : EndpointAddress::Socket (net::SocketAddr::from (([127,0,0,1], 8080))),
				protocol : EndpointProtocol::Http1,
			}
	}
}

