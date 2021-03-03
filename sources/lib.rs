

#![ no_implicit_prelude ]




#![ allow (warnings) ]


#![ warn (absolute_paths_not_starting_with_crate) ]
#![ warn (anonymous_parameters) ]
#![ warn (elided_lifetimes_in_paths) ]
#![ warn (explicit_outlives_requirements) ]
#![ warn (invalid_html_tags) ]
#![ warn (keyword_idents) ]
#![ warn (macro_use_extern_crate) ]
#![ warn (meta_variable_misuse) ]
//#![ warn (missing_crate_level_docs) ]
//#![ warn (missing_doc_code_examples) ]
//#![ warn (missing_docs) ]
#![ warn (non_ascii_idents) ]
#![ warn (pointer_structural_match) ]
#![ warn (private_doc_tests) ]
#![ warn (single_use_lifetimes) ]
#![ warn (trivial_casts) ]
#![ warn (trivial_numeric_casts) ]
//#![ warn (unreachable_pub) ]
#![ warn (unsafe_code) ]
#![ warn (unused_crate_dependencies) ]
#![ warn (unused_extern_crates) ]
#![ warn (unused_import_braces) ]
#![ warn (unused_lifetimes) ]
#![ warn (unused_qualifications) ]
#![ warn (variant_size_differences) ]

#![ allow (box_pointers) ]
#![ allow (missing_copy_implementations) ]
#![ allow (missing_debug_implementations) ]
#![ allow (unused_results) ]


#![ warn (clippy::all) ]
#![ warn (clippy::correctness) ]
#![ warn (clippy::style) ]
#![ warn (clippy::complexity) ]
#![ warn (clippy::perf) ]
#![ warn (clippy::cargo) ]
#![ allow (clippy::pedantic) ]
#![ allow (clippy::nursery) ]

#![ allow (clippy::unused_unit) ]
#![ allow (clippy::new_without_default) ]

#![ allow (clippy::cargo_common_metadata) ]
#![ allow (clippy::wildcard_dependencies) ]


#![ cfg_attr (not (feature = "hss-full"), allow (unused_imports)) ]




pub use {
	crate::accepter::*,
	crate::cli::*,
	crate::connection::*,
	crate::configuration::*,
	crate::errors::exports::*,
	crate::extensions::*,
	crate::handler::*,
	crate::main::*,
	crate::routes::*,
	crate::server::*,
};


#[ cfg (feature = "hss-exports") ]
pub use {
	crate::exports::*,
	crate::dependencies::*,
};


#[ cfg (feature = "hss-internals") ]
pub mod internals {
	#![ allow (unused_import_braces) ]
	pub use {
		crate::errors::internals::*,
	};
}




pub(crate) mod accepter;
pub(crate) mod cli;
pub(crate) mod configuration;
pub(crate) mod connection;
pub(crate) mod errors;
pub(crate) mod exports;
pub(crate) mod extensions;
pub(crate) mod handler;
pub(crate) mod main;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod server;


#[ cfg (all (feature = "hss-server", not (feature = "hss-server-http"), not (feature = "features-fuzzing"))) ]
compile_error! ("enable any of HTTP/1 or HTTP/2");




#[ cfg (feature = "hss-jemalloc") ]
#[global_allocator]
static ALLOCATOR : ::jemallocator::Jemalloc = jemallocator::Jemalloc;




mod dependencies {
	
	#[ cfg (feature = "hyper") ]
	pub use ::hyper;
	
	#[ cfg (feature = "tokio") ]
	pub use ::tokio;
	
	#[ cfg (feature = "http") ]
	pub use ::http;
	
	#[ cfg (feature = "http-body") ]
	pub use ::http_body;
	
	#[ cfg (feature = "bytes") ]
	pub use ::bytes;
	
	
	#[ cfg (feature = "rustls") ]
	pub use ::rustls;
	
	#[ cfg (feature = "tokio-rustls") ]
	pub use ::tokio_rustls;
	
	#[ cfg (feature = "rustls-pemfile") ]
	pub use ::rustls_pemfile;
	
	
	#[ cfg (feature = "native-tls") ]
	pub use ::native_tls;
	
	#[ cfg (feature = "tokio-native-tls") ]
	pub use ::tokio_native_tls;
	
	
	#[ allow (unused_imports) ]
	#[ cfg (feature = "futures") ]
	pub(crate) use ::futures;
	
	#[ allow (unused_imports) ]
	#[ cfg (feature = "path-tree") ]
	pub(crate) use ::path_tree;
	
	#[ allow (unused_imports) ]
	#[ cfg (feature = "argparse") ]
	pub(crate) use ::argparse;
	
	#[ allow (unused_imports) ]
	#[ cfg (feature = "jemallocator") ]
	pub(crate) use ::jemallocator;
}

// NOTE:  Required so that `cargo docs` doesn't break...
#[ allow (unused_imports) ]
use crate::dependencies::*;

