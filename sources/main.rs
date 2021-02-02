

use crate::prelude::*;




pub fn main () -> () {
	
	let _configuration = Configuration::default ();
	
	let _server = Server::new (_configuration) .or_panic (0x0e71d6cc);
	_server.run () .or_panic (0x5116af5e);
}

