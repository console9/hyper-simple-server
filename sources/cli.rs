

use crate::prelude::*;




#[ derive (Default) ]
pub struct ConfigurationArguments {
	
	pub endpoint_socket_address : Option<String>,
	pub endpoint_socket_address_help : String,
	
	#[ cfg (unix) ]
	pub endpoint_descriptor : Option<u32>,
	#[ cfg (unix) ]
	pub endpoint_descriptor_help : String,
	
	pub endpoint_protocol_http1 : Option<bool>,
	pub endpoint_protocol_http1_help : String,
	pub endpoint_protocol_http2 : Option<bool>,
	pub endpoint_protocol_http2_help : String,
	
	pub endpoint_insecure : Option<bool>,
	pub endpoint_insecure_help : String,
	
	pub endpoint_rust_tls_certificate_pem_path : Option<String>,
	pub endpoint_rust_tls_certificate_pem_path_help : String,
	
	pub endpoint_native_tls_certificate_pkcs12_path : Option<String>,
	pub endpoint_native_tls_certificate_pkcs12_password : Option<String>,
	pub endpoint_native_tls_certificate_pkcs12_path_help : String,
}




impl ConfigurationArguments {
	
	pub fn with_defaults (_configuration : &Configuration) -> Self {
		
		let mut _arguments = Self::default ();
		
		match _configuration.endpoint.address {
			EndpointAddress::Socket (_address) =>
				_arguments.endpoint_socket_address = Some (_address.to_string ()),
			#[ cfg (unix) ]
			EndpointAddress::Descriptor (_descriptor) =>
				_arguments.endpoint_descriptor = Some (_descriptor),
		}
		
		match _configuration.endpoint.protocol {
			EndpointProtocol::Http1 =>
				_arguments.endpoint_protocol_http1 = Some (true),
			EndpointProtocol::Http2 =>
				_arguments.endpoint_protocol_http2 = Some (true),
			EndpointProtocol::Http12 => {
				_arguments.endpoint_protocol_http1 = Some (true);
				_arguments.endpoint_protocol_http2 = Some (true);
			}
			EndpointProtocol::Generic =>
				(),
		}
		
		match _configuration.endpoint.security {
			EndpointSecurity::Insecure =>
				_arguments.endpoint_insecure = Some (true),
			EndpointSecurity::RustTls (_) =>
				(),
			EndpointSecurity::NativeTls (_) =>
				(),
		}
		
		_arguments
	}
	
	
	#[ allow (single_use_lifetimes) ]
	pub fn prepare <'a> (&'a mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		self.endpoint_socket_address_help = self.endpoint_socket_address.as_ref () .map_or_else (
				|| format! ("listen on TCP socket address"),
				|_address| format! ("listen on TCP socket address (default `{}`)", _address));
		_parser.refer (&mut self.endpoint_socket_address)
				.metavar ("<socket-address>")
				.add_option (&["--listen-address"], argparse::StoreOption, &self.endpoint_socket_address_help)
				.add_option (&["--listen-any-80"], argparse::StoreConst (Some (String::from ("0.0.0.0:80"))), "listen on any IP with port 80 (might require root or capabilities)")
				.add_option (&["--listen-any-443"], argparse::StoreConst (Some (String::from ("0.0.0.0:443"))), "listen on any IP with port 443 (might require root or capabilities)")
				.add_option (&["--listen-any-8080"], argparse::StoreConst (Some (String::from ("0.0.0.0:8080"))), "listen on any IP with port 8080")
				.add_option (&["--listen-any-8443"], argparse::StoreConst (Some (String::from ("0.0.0.0:8443"))), "listen on any IP with port 8443")
				.add_option (&["--listen-localhost-8080"], argparse::StoreConst (Some (String::from ("127.0.0.1:8080"))), "listen on localhost with port 8080")
				.add_option (&["--listen-localhost-8443"], argparse::StoreConst (Some (String::from ("127.0.0.1:8443"))), "listen on localhost with port 8443");
		
		#[ cfg (unix) ]
		{
		self.endpoint_descriptor_help = self.endpoint_descriptor.as_ref () .map_or_else (
				|| format! ("listen on TCP socket with descriptor"),
				|_descriptor| format! ("listen on TCP socket with descriptor (default `{}`)", _descriptor));
		_parser.refer (&mut self.endpoint_descriptor)
				.metavar ("<socket-descriptor>")
				.add_option (&["--listen-descriptor"], argparse::StoreOption, &self.endpoint_descriptor_help);
		}
		
		self.endpoint_protocol_http1_help = self.endpoint_protocol_http1.as_ref () .map_or_else (
				|| format! ("enable HTTP/1 support"),
				|_enabled| format! ("enable HTTP/1 support (default `{}`)", _enabled));
		_parser.refer (&mut self.endpoint_protocol_http1)
				.add_option (&["--enable-http1"], argparse::StoreConst (Some (true)), &self.endpoint_protocol_http1_help)
				.add_option (&["--disable-http1"], argparse::StoreConst (Some (false)), "");
		
		self.endpoint_protocol_http2_help = self.endpoint_protocol_http2.as_ref () .map_or_else (
				|| format! ("enable HTTP/2 support"),
				|_enabled| format! ("enable HTTP/2 support (default `{}`)", _enabled));
		_parser.refer (&mut self.endpoint_protocol_http2)
				.add_option (&["--enable-http2"], argparse::StoreConst (Some (true)), &self.endpoint_protocol_http2_help)
				.add_option (&["--disable-http2"], argparse::StoreConst (Some (false)), "");
		
		self.endpoint_insecure_help = self.endpoint_insecure.as_ref () .map_or_else (
				|| format! ("disable TLS support"),
				|_enabled| format! ("disable TLS support (default `{}`)", _enabled));
		_parser.refer (&mut self.endpoint_insecure)
				.add_option (&["--disable-tls"], argparse::StoreConst (Some (false)), &self.endpoint_insecure_help)
				.add_option (&["--enable-tls"], argparse::StoreConst (Some (true)), "");
		
		self.endpoint_rust_tls_certificate_pem_path_help = self.endpoint_rust_tls_certificate_pem_path.as_ref () .map_or_else (
				|| format! ("load TLS certificate in PEM format (with Rust TLS library) from path"),
				|_path| format! ("load TLS certificate in PEM format (with Rust TLS library) from path (default `{}`)", _path));
		_parser.refer (&mut self.endpoint_rust_tls_certificate_pem_path)
				.metavar ("<path>")
				.add_option (&["--load-rust-tls-pem-path"], argparse::StoreOption, &self.endpoint_rust_tls_certificate_pem_path_help);
		
		self.endpoint_native_tls_certificate_pkcs12_path_help = self.endpoint_native_tls_certificate_pkcs12_path.as_ref () .map_or_else (
				|| format! ("load TLS certificate in PKCS#12 format (with native TLS library) from path"),
				|_path| format! ("load TLS certificate in PKCS#12 format (with native TLS library) from path (default `{}`)", _path));
		_parser.refer (&mut self.endpoint_native_tls_certificate_pkcs12_path)
				.metavar ("<path>")
				.add_option (&["--load-native-tls-pkcs12-path"], argparse::StoreOption, &self.endpoint_native_tls_certificate_pkcs12_path_help);
		_parser.refer (&mut self.endpoint_native_tls_certificate_pkcs12_password)
				.metavar ("<password>")
				.add_option (&["--load-native-tls-pkcs12-password"], argparse::StoreOption, "");
	}
	
	
	pub fn update (&self, _configuration : &mut Configuration) -> ServerResult {
		
		#[ cfg (unix) ]
		if self.endpoint_socket_address.is_some () && self.endpoint_descriptor.is_some () {
			return Err (error_with_message (0xbb9b6c08, "conflicting TCP listen options specified"));
		}
		
		if let Some (_address) = self.endpoint_socket_address.as_ref () {
			_configuration.endpoint.address = EndpointAddress::from_socket_address_parse (_address) ?;
		}
		#[ cfg (unix) ]
		if let Some (_descriptor) = self.endpoint_descriptor {
			_configuration.endpoint.address = EndpointAddress::from_descriptor (_descriptor);
		}
		
		let mut _http1_enabled = _configuration.endpoint.protocol.supports_http1 ();
		if let Some (_enabled) = self.endpoint_protocol_http1 {
			_http1_enabled = _enabled;
		}
		let mut _http2_enabled = _configuration.endpoint.protocol.supports_http2 ();
		if let Some (_enabled) = self.endpoint_protocol_http2 {
			_http2_enabled = _enabled;
		}
		_configuration.endpoint.protocol = EndpointProtocol::with_http_support (_http1_enabled, _http2_enabled);
		
		if self.endpoint_rust_tls_certificate_pem_path.is_some () && self.endpoint_native_tls_certificate_pkcs12_path.is_some () {
			return Err (error_with_message (0x7ce8d799, "conflicting load TLS certificate options specified"));
		}
		if let Some (_path) = self.endpoint_rust_tls_certificate_pem_path.as_ref () {
			_configuration.endpoint.security = EndpointSecurity::RustTls (RustTlsCertificate::load_from_pem_file (_path) ?);
		}
		if let Some (_path) = self.endpoint_native_tls_certificate_pkcs12_path.as_ref () {
			let _password = self.endpoint_native_tls_certificate_pkcs12_password.as_ref () .map_or_else (|| "", String::as_str);
			_configuration.endpoint.security = EndpointSecurity::NativeTls (NativeTlsCertificate::load_from_pkcs12_file (_path, _password) ?);
		}
		
		Ok (())
	}
	
	
	pub fn parse (&mut self) -> ServerResult {
		
		let mut _parser = argparse::ArgumentParser::new ();
		self.prepare (&mut _parser);
		_parser.parse_args_or_exit ();
		
		Ok (())
	}
	
	
	pub fn parse_and_update (_configuration : &mut Configuration) -> ServerResult {
		
		let mut _arguments = Self::with_defaults (_configuration);
		_arguments.parse () ?;
		_arguments.update (_configuration) ?;
		
		Ok (())
	}
}

