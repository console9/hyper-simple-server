

use crate::prelude::*;




#[ derive (Default) ]
#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
pub struct ConfigurationArguments {
	
	pub endpoint_socket_address : Option<String>,
	pub endpoint_socket_address_help : String,
	
	#[ cfg (unix) ]
	pub endpoint_descriptor : Option<u32>,
	#[ cfg (unix) ]
	pub endpoint_descriptor_help : String,
	
	#[ cfg (feature = "hyper--server-http1") ]
	pub endpoint_protocol_http1 : Option<bool>,
	#[ cfg (feature = "hyper--server-http1") ]
	pub endpoint_protocol_http1_help : String,
	#[ cfg (feature = "hyper--server-http2") ]
	pub endpoint_protocol_http2 : Option<bool>,
	#[ cfg (feature = "hyper--server-http2") ]
	pub endpoint_protocol_http2_help : String,
	
	#[ cfg (feature = "hss-tls-any") ]
	pub endpoint_insecure : Option<bool>,
	#[ cfg (feature = "hss-tls-any") ]
	pub endpoint_insecure_help : String,
	
	#[ cfg (feature = "hss-tls-rust") ]
	pub endpoint_rust_tls_certificate_pem_path : Option<String>,
	#[ cfg (feature = "hss-tls-rust") ]
	pub endpoint_rust_tls_certificate_pem_path_help : String,
	#[ cfg (feature = "hss-tls-rust") ]
	pub endpoint_rust_tls_certificate_fallback : Option<RustTlsCertificate>,
	
	#[ cfg (feature = "hss-tls-native") ]
	pub endpoint_native_tls_certificate_pkcs12_path : Option<String>,
	#[ cfg (feature = "hss-tls-native") ]
	pub endpoint_native_tls_certificate_pkcs12_password : Option<String>,
	#[ cfg (feature = "hss-tls-native") ]
	pub endpoint_native_tls_certificate_pkcs12_path_help : String,
	#[ cfg (feature = "hss-tls-native") ]
	pub endpoint_native_tls_certificate_fallback : Option<NativeTlsCertificate>,
	
	#[ cfg (feature = "hss-server-mt") ]
	pub server_threads : Option<usize>,
	#[ cfg (feature = "hss-server-mt") ]
	pub server_threads_help : String,
	
	#[ cfg (feature = "hss-server-profiling") ]
	pub server_profiling : Option<String>,
	#[ cfg (feature = "hss-server-profiling") ]
	pub server_profiling_help : String,
}




#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
impl ConfigurationArguments {
	
	pub fn with_defaults (_configuration : &Configuration) -> CliResult<Self> {
		
		let mut _arguments = Self::default ();
		
		match _configuration.endpoint.address {
			EndpointAddress::Socket (_address) =>
				_arguments.endpoint_socket_address = Some (_address.to_string ()),
			#[ cfg (unix) ]
			EndpointAddress::Descriptor (_descriptor) =>
				_arguments.endpoint_descriptor = Some (_descriptor),
		}
		
		match _configuration.endpoint.protocol {
			#[ cfg (feature = "hyper--server-http1") ]
			EndpointProtocol::Http1 =>
				_arguments.endpoint_protocol_http1 = Some (true),
			#[ cfg (feature = "hyper--server-http2") ]
			EndpointProtocol::Http2 =>
				_arguments.endpoint_protocol_http2 = Some (true),
			#[ cfg (feature = "hyper--server-http1") ]
			#[ cfg (feature = "hyper--server-http2") ]
			EndpointProtocol::Http12 => {
				_arguments.endpoint_protocol_http1 = Some (true);
				_arguments.endpoint_protocol_http2 = Some (true);
			}
			EndpointProtocol::Generic =>
				(),
		}
		
		#[ cfg (feature = "hss-tls-any") ]
		match _configuration.endpoint.security {
			EndpointSecurity::Insecure =>
				_arguments.endpoint_insecure = Some (true),
			#[ cfg (feature = "hss-tls-rust") ]
			EndpointSecurity::RustTls (ref _certificate) => {
				_arguments.endpoint_insecure = Some (false);
				_arguments.endpoint_rust_tls_certificate_fallback = Some (_certificate.clone ());
			}
			#[ cfg (feature = "hss-tls-native") ]
			EndpointSecurity::NativeTls (ref _certificate) => {
				_arguments.endpoint_insecure = Some (false);
				_arguments.endpoint_native_tls_certificate_fallback = Some (_certificate.clone ());
			}
		}
		
		#[ cfg (feature = "hss-server-mt") ]
		{
		_arguments.server_threads = _configuration.threads;
		}
		
		#[ cfg (feature = "hss-server-profiling") ]
		if let Some (_path) = _configuration.profiling.as_ref () {
			let _path = _path.to_str () .else_wrap (0xd708ca76) ?;
			_arguments.server_profiling = Some (_path.to_owned ());
		}
		
		Ok (_arguments)
	}
	
	
	#[ allow (single_use_lifetimes) ]
	pub fn prepare <'a> (&'a mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		self.endpoint_socket_address_help = self.endpoint_socket_address.as_ref () .map_or_else (
				|| format! ("listen on TCP socket address"),
				|_address| format! ("listen on TCP socket address (default `{}`)", _address));
		let mut _argument = _parser.refer (&mut self.endpoint_socket_address);
		{
			_argument
				.metavar ("<socket-address>")
				.add_option (&["--listen-address"], argparse::StoreOption, &self.endpoint_socket_address_help)
				.add_option (&["--listen-any-80"], argparse::StoreConst (Some (String::from ("0.0.0.0:80"))), "listen on any IP with port 80 (might require root or capabilities)")
				.add_option (&["--listen-any-8080"], argparse::StoreConst (Some (String::from ("0.0.0.0:8080"))), "listen on any IP with port 8080")
				.add_option (&["--listen-localhost-8080"], argparse::StoreConst (Some (String::from ("127.0.0.1:8080"))), "listen on localhost with port 8080");
		}
		#[ cfg (feature = "hss-tls-any") ]
		{
			_argument
				.add_option (&["--listen-any-443"], argparse::StoreConst (Some (String::from ("0.0.0.0:443"))), "listen on any IP with port 443 (might require root or capabilities)")
				.add_option (&["--listen-any-8443"], argparse::StoreConst (Some (String::from ("0.0.0.0:8443"))), "listen on any IP with port 8443")
				.add_option (&["--listen-localhost-8443"], argparse::StoreConst (Some (String::from ("127.0.0.1:8443"))), "listen on localhost with port 8443")
			;
		}
		
		#[ cfg (unix) ]
		{
		self.endpoint_descriptor_help = self.endpoint_descriptor.as_ref () .map_or_else (
				|| format! ("listen on TCP socket with descriptor"),
				|_descriptor| format! ("listen on TCP socket with descriptor (default `{}`)", _descriptor));
		_parser.refer (&mut self.endpoint_descriptor)
				.metavar ("<socket-descriptor>")
				.add_option (&["--listen-descriptor"], argparse::StoreOption, &self.endpoint_descriptor_help);
		}
		
		#[ cfg (feature = "hyper--server-http1") ]
		{
		self.endpoint_protocol_http1_help = self.endpoint_protocol_http1.as_ref () .map_or_else (
				|| format! ("enable HTTP/1 support"),
				|_enabled| format! ("enable HTTP/1 support (default `{}`)", _enabled));
		_parser.refer (&mut self.endpoint_protocol_http1)
				.add_option (&["--enable-http1"], argparse::StoreConst (Some (true)), &self.endpoint_protocol_http1_help)
				.add_option (&["--disable-http1"], argparse::StoreConst (Some (false)), "");
		}
		
		#[ cfg (feature = "hyper--server-http2") ]
		{
		self.endpoint_protocol_http2_help = self.endpoint_protocol_http2.as_ref () .map_or_else (
				|| format! ("enable HTTP/2 support"),
				|_enabled| format! ("enable HTTP/2 support (default `{}`)", _enabled));
		_parser.refer (&mut self.endpoint_protocol_http2)
				.add_option (&["--enable-http2"], argparse::StoreConst (Some (true)), &self.endpoint_protocol_http2_help)
				.add_option (&["--disable-http2"], argparse::StoreConst (Some (false)), "");
		}
		
		#[ cfg (feature = "hss-tls-any") ]
		{
		self.endpoint_insecure_help = if self.endpoint_insecure.unwrap_or (false) {
				format! ("disable TLS support (default disabled)")
			} else {
				format! ("disable TLS support (default enabled)")
			};
		_parser.refer (&mut self.endpoint_insecure)
				.add_option (&["--disable-tls"], argparse::StoreConst (Some (true)), &self.endpoint_insecure_help)
				.add_option (&["--enable-tls"], argparse::StoreConst (Some (false)), "");
		}
		
		#[ cfg (feature = "hss-tls-rust") ]
		{
		let _endpoint_has_certificate_fallback = self.endpoint_rust_tls_certificate_fallback.is_some ();
		self.endpoint_rust_tls_certificate_pem_path_help = self.endpoint_rust_tls_certificate_pem_path.as_ref () .map_or_else (
				|| if _endpoint_has_certificate_fallback {
					format! ("load TLS certificate in PEM format (with Rust TLS library) from path (default embedded in binary)")
				} else {
					format! ("load TLS certificate in PEM format (with Rust TLS library) from path")
				},
				|_path| format! ("load TLS certificate in PEM format (with Rust TLS library) from path (default `{}`)", _path));
		_parser.refer (&mut self.endpoint_rust_tls_certificate_pem_path)
				.metavar ("<path>")
				.add_option (&["--load-rust-tls-pem-path"], argparse::StoreOption, &self.endpoint_rust_tls_certificate_pem_path_help);
		}
		
		#[ cfg (feature = "hss-tls-native") ]
		{
		let _endpoint_has_certificate_fallback = self.endpoint_native_tls_certificate_fallback.is_some ();
		self.endpoint_native_tls_certificate_pkcs12_path_help = self.endpoint_native_tls_certificate_pkcs12_path.as_ref () .map_or_else (
				|| if _endpoint_has_certificate_fallback {
					format! ("load TLS certificate in PKCS#12 format (with native TLS library) from path (default embedded in binary)")
				} else {
					format! ("load TLS certificate in PKCS#12 format (with native TLS library) from path")
				},
				|_path| format! ("load TLS certificate in PKCS#12 format (with native TLS library) from path (default `{}`)", _path));
		_parser.refer (&mut self.endpoint_native_tls_certificate_pkcs12_path)
				.metavar ("<path>")
				.add_option (&["--load-native-tls-pkcs12-path"], argparse::StoreOption, &self.endpoint_native_tls_certificate_pkcs12_path_help);
		_parser.refer (&mut self.endpoint_native_tls_certificate_pkcs12_password)
				.metavar ("<password>")
				.add_option (&["--load-native-tls-pkcs12-password"], argparse::StoreOption, "");
		}
		
		#[ cfg (feature = "hss-server-mt") ]
		{
		self.server_threads_help = self.server_threads.as_ref () .map_or_else (
				|| format! ("enable server multi-threading"),
				|_threads| format! ("enable server multi-threading (default `{}` threads)", _threads));
		_parser.refer (&mut self.server_threads)
				.metavar ("<server-threads>")
				.add_option (&["--server-threads"], argparse::StoreOption, &self.server_threads_help)
				.add_option (&["--no-server-threads"], argparse::StoreConst (None), "");
		}
		
		#[ cfg (feature = "hss-server-profiling") ]
		{
		self.server_profiling_help = self.server_profiling.as_ref () .map_or_else (
				|| format! ("enable server profiling"),
				|_profiling| format! ("enable server profiling (default `{}` path)", _profiling));
		_parser.refer (&mut self.server_profiling)
				.metavar ("<server-profiling>")
				.add_option (&["--server-profiling"], argparse::StoreOption, &self.server_profiling_help)
				.add_option (&["--no-server-profiling"], argparse::StoreConst (None), "");
		}
	}
	
	
	pub fn update (&self, _configuration : &mut Configuration) -> CliResult {
		
		#[ cfg (unix) ]
		if self.endpoint_socket_address.is_some () && self.endpoint_descriptor.is_some () {
			fail! (0xbb9b6c08, "conflicting TCP listen options specified");
		}
		
		if let Some (_address) = self.endpoint_socket_address.as_ref () {
			_configuration.endpoint.address = EndpointAddress::from_socket_address_parse (_address) .else_wrap (0xa7dd30b6) ?;
		}
		#[ cfg (unix) ]
		if let Some (_descriptor) = self.endpoint_descriptor {
			_configuration.endpoint.address = EndpointAddress::from_descriptor (_descriptor);
		}
		
		#[ cfg (feature = "hyper--server-http") ]
		{
		let mut _http1_enabled = _configuration.endpoint.protocol.supports_http1 ();
		#[ cfg (feature = "hyper--server-http1") ]
		if let Some (_enabled) = self.endpoint_protocol_http1 {
			_http1_enabled = _enabled;
		}
		let mut _http2_enabled = _configuration.endpoint.protocol.supports_http2 ();
		#[ cfg (feature = "hyper--server-http2") ]
		if let Some (_enabled) = self.endpoint_protocol_http2 {
			_http2_enabled = _enabled;
		}
		_configuration.endpoint.protocol = EndpointProtocol::with_http_support (_http1_enabled, _http2_enabled);
		}
		
		#[ cfg (feature = "hss-tls-any") ]
		if let Some (true) = self.endpoint_insecure {
			_configuration.endpoint.security = EndpointSecurity::Insecure;
		}
		
		#[ cfg (feature = "hss-tls-rust") ]
		#[ cfg (feature = "hss-tls-native") ]
		if self.endpoint_rust_tls_certificate_pem_path.is_some () && self.endpoint_native_tls_certificate_pkcs12_path.is_some () {
			fail! (0x7ce8d799, "conflicting load TLS certificate options specified");
		}
		#[ cfg (feature = "hss-tls-rust") ]
		if let Some (_path) = self.endpoint_rust_tls_certificate_pem_path.as_ref () {
			_configuration.endpoint.security = EndpointSecurity::RustTls (RustTlsCertificate::load_from_pem_file (_path) .else_wrap (0x1676bffe) ?);
		} else if let Some (_certificate) = self.endpoint_rust_tls_certificate_fallback.as_ref () {
			if let Some (false) = self.endpoint_insecure {
				_configuration.endpoint.security = EndpointSecurity::RustTls (_certificate.clone ());
			}
		}
		#[ cfg (feature = "hss-tls-native") ]
		if let Some (_path) = self.endpoint_native_tls_certificate_pkcs12_path.as_ref () {
			let _password = self.endpoint_native_tls_certificate_pkcs12_password.as_ref () .map_or_else (|| "", String::as_str);
			_configuration.endpoint.security = EndpointSecurity::NativeTls (NativeTlsCertificate::load_from_pkcs12_file (_path, _password) ?);
		} else if let Some (_certificate) = self.endpoint_native_tls_certificate_fallback.as_ref () {
			if let Some (false) = self.endpoint_insecure {
				_configuration.endpoint.security = EndpointSecurity::NativeTls (_certificate.clone ());
			}
		}
		
		#[ cfg (feature = "hss-tls-any") ]
		if let Some (_endpoint_insecure) = self.endpoint_insecure {
			if _endpoint_insecure {
				if let EndpointSecurity::Insecure = _configuration.endpoint.security {
					// NOP
				} else {
					fail! (0x1111c2cc, "conflicting insecure and load TLS certificate options");
				}
			} else {
				if let EndpointSecurity::Insecure = _configuration.endpoint.security {
					#[ cfg (feature = "hss-tls-any") ]
					fail! (0x6621c453, "conflicting secure and missing load TLS certificate options");
					#[ cfg (not (feature = "hss-tls-any")) ]
					fail! (0x0e0edc6a, "conflicting secure and unavailable TLS engine options");
				} else {
					// NOP
				}
			}
		}
		
		#[ cfg (feature = "hss-server-mt") ]
		{
		_configuration.threads = self.server_threads;
		}
		
		#[ cfg (feature = "hss-server-profiling") ]
		{
		_configuration.profiling = self.server_profiling.as_ref () .map (|_path| path::PathBuf::from (_path));
		}
		
		Ok (())
	}
	
	
	pub fn parse (_configuration : Configuration, _arguments : Option<CliArguments>) -> CliResult<Configuration> {
		Self::parse_with_extensions (_configuration, (), _arguments)
	}
	
	pub fn parse_with_extensions (mut _configuration : Configuration, mut _extensions : impl CliExtensions, _arguments : Option<CliArguments>) -> CliResult<Configuration> {
		
		let _arguments = CliArguments::unwrap_or_args (_arguments);
		let mut _arguments = _arguments.into_vec_str ();
		_arguments.insert (0, "<cli>".into ());
		
		let mut _self = Self::with_defaults (&_configuration) ?;
		
		{
			let mut _parser = argparse::ArgumentParser::new ();
			_self.prepare (&mut _parser);
			_extensions.prepare (&mut _parser);
			match _parser.parse (_arguments, &mut io::stdout (), &mut io::stderr ()) {
				Ok (()) =>
					(),
				Err (0) =>
					::std::process::exit (0),
				Err (_code) =>
					fail! (0x4fec67d5, "invalid arguments!"),
			}
		}
		
		_self.update (&mut _configuration) ?;
		
		Ok (_configuration)
	}
}


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
pub trait CliExtensions {
	
	fn prepare <'a> (self, _parser : &mut argparse::ArgumentParser<'a>) -> () where Self : 'a;
}

#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
impl CliExtensions for () {
	
	fn prepare <'a> (self, _parser : &mut argparse::ArgumentParser<'a>) -> () where Self : 'a {}
}


#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
pub enum CliArgument<'a> {
	String (&'a mut String, &'static str, &'static str),
	StringConst (&'a mut String, &'a mut str, &'static str, &'static str),
	Boolean (&'a mut bool, &'static str, &'static str),
	BooleanConst (&'a mut bool, bool, &'static str, &'static str),
}

#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
impl CliExtensions for CliArgument<'_> {
	
	fn prepare <'a> (self, _parser : &mut argparse::ArgumentParser<'a>) -> () where Self : 'a {
		match self {
			CliArgument::String (_variable, _flag, _help) => {
				_parser.refer (_variable) .metavar ("<string>") .add_option (&[_flag], argparse::Store, _help);
			}
			CliArgument::StringConst (_variable, _value, _flag, _help) => {
				_parser.refer (_variable) .metavar ("<string>") .add_option (&[_flag], argparse::StoreConst (_value.into ()), _help);
			}
			CliArgument::Boolean (_variable, _flag, _help) => {
				_parser.refer (_variable) .metavar ("<boolean>") .add_option (&[_flag], argparse::Store, _help);
			}
			CliArgument::BooleanConst (_variable, _value, _flag, _help) => {
				_parser.refer (_variable) .metavar ("<boolean>") .add_option (&[_flag], argparse::StoreConst (_value), _help);
			}
		}
	}
}

#[ cfg (feature = "hss-config") ]
#[ cfg (feature = "hss-cli") ]
impl <const N : usize> CliExtensions for [CliArgument<'_>; N] {
	
	fn prepare <'a> (self, _parser : &mut argparse::ArgumentParser<'a>) -> () where Self : 'a {
		for _argument in self {
			_argument.prepare (_parser);
		}
	}
}




#[ cfg (feature = "hss-cli") ]
pub struct CliArguments (Vec<OsString>);

#[ cfg (not (feature = "hss-cli")) ]
pub struct CliArguments {
	_private : (),
}


#[ cfg (feature = "hss-cli") ]
impl CliArguments {
	
	pub fn unwrap_or_args (_arguments : Option<CliArguments>) -> CliArguments {
		_arguments.unwrap_or_else (CliArguments::from_args)
	}
	
	pub fn from_args () -> CliArguments {
		CliArguments (env::args_os () .into_iter () .skip (1) .collect ())
	}
	
	pub fn from_vec_os (_arguments : Vec<OsString>) -> CliArguments {
		CliArguments (_arguments)
	}
	
	pub fn from_vec_str (_arguments : Vec<String>) -> CliArguments {
		CliArguments::from_vec_os (_arguments.into_iter () .map (OsString::from) .collect ())
	}
	
	pub fn into_vec_os (self) -> Vec<OsString> {
		self.0
	}
	
	pub fn into_vec_str (self) -> Vec<String> {
		self.into_vec_os ()
				.into_iter ()
				.map (OsString::into_string)
				.map (|_result| _result.unwrap_or_else (|_string| _string.to_string_lossy () .into_owned ()))
				.collect ()
	}
	
	pub fn is_empty (&self) -> bool {
		self.0.is_empty ()
	}
	
	pub fn first_os (&self) -> Option<&OsStr> {
		self.0.first () .map (|_string| _string.as_ref ())
	}
	
	pub fn first_str (&self) -> Option<&str> {
		self.0.first () .and_then (|_string| _string.to_str ())
	}
	
	pub fn remove_first_os (&mut self) -> Option<OsString> {
		if self.0.is_empty () {
			return None;
		}
		Some (self.0.remove (0))
	}
	
	pub fn remove_first_str (&mut self) -> Option<String> {
		self.remove_first_os ()
				.map (OsString::into_string)
				.map (|_result| _result.unwrap_or_else (|_string| _string.to_string_lossy () .into_owned ()))
	}
	
	pub fn without_first (mut self) -> Self {
		self.remove_first_os ();
		self
	}
	
	pub fn into_tuple_n_os (self, _length : usize) -> Result<Vec<OsString>, Self> {
		if self.0.len () == _length {
			Ok (self.into_vec_os ())
		} else {
			Err (self)
		}
	}
	
	pub fn into_tuple_n_str (self, _length : usize) -> Result<Vec<String>, Self> {
		if self.0.len () == _length {
			Ok (self.into_vec_str ())
		} else {
			Err (self)
		}
	}
	
	pub fn into_tuple_1_os (self) -> Result<(OsString,), Self> {
		let mut _elements = self.into_tuple_n_os (1) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0x0db4f89f),
			);
		Ok (_tuple)
	}
	
	pub fn into_tuple_1_str (self) -> Result<(String,), Self> {
		let mut _elements = self.into_tuple_n_str (1) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0x3e69a37d),
			);
		Ok (_tuple)
	}
	
	pub fn into_tuple_2_os (self) -> Result<(OsString, OsString), Self> {
		let mut _elements = self.into_tuple_n_os (2) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0x7303f446),
				_elements.next () .infallible (0x1d266eeb),
			);
		Ok (_tuple)
	}
	
	pub fn into_tuple_2_str (self) -> Result<(String, String), Self> {
		let mut _elements = self.into_tuple_n_str (2) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0x8ba4b0b2),
				_elements.next () .infallible (0x3507fde7),
			);
		Ok (_tuple)
	}
	
	pub fn into_tuple_3_os (self) -> Result<(OsString, OsString, OsString), Self> {
		let mut _elements = self.into_tuple_n_os (3) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0xdc3beb9b),
				_elements.next () .infallible (0x90b81bd1),
				_elements.next () .infallible (0xfb6535ec),
			);
		Ok (_tuple)
	}
	
	pub fn into_tuple_3_str (self) -> Result<(String, String, String), Self> {
		let mut _elements = self.into_tuple_n_str (3) ? .into_iter ();
		let _tuple = (
				_elements.next () .infallible (0x76903952),
				_elements.next () .infallible (0xfbb72b5b),
				_elements.next () .infallible (0x404f94ba),
			);
		Ok (_tuple)
	}
}

