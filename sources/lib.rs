

#![ no_implicit_prelude ]




#![ allow (warnings) ]
#![ cfg_attr (feature = "features-fuzzing", deny (warnings)) ]


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
	crate::errors::*,
	crate::extensions::*,
	crate::resources::*,
	crate::handler::*,
	crate::main::*,
	crate::routes::*,
	crate::sanitize::*,
	crate::server::*,
	crate::profiling::*,
};


#[ cfg (feature = "hss-exports") ]
pub use {
	crate::exports::*,
	crate::dependencies::*,
};




pub(crate) mod accepter;
pub(crate) mod cli;
pub(crate) mod configuration;
pub(crate) mod connection;
pub(crate) mod exports;
pub(crate) mod extensions;
pub(crate) mod resources;
pub(crate) mod handler;
pub(crate) mod main;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod sanitize;
pub(crate) mod server;
pub(crate) mod profiling;

pub mod errors;


#[ cfg (all (feature = "hss-server-core", not (feature = "hyper--server-http"), not (feature = "features-fuzzing"))) ]
compile_error! ("enable any of HTTP/1 or HTTP/2");




#[ cfg (all (feature = "hss-jemalloc", not (feature = "hss-mimalloc"))) ]
#[global_allocator]
static ALLOCATOR : ::jemallocator::Jemalloc = ::jemallocator::Jemalloc;

#[ cfg (all (feature = "hss-mimalloc", not (feature = "hss-jemalloc"))) ]
#[global_allocator]
static ALLOCATOR : ::mimalloc::MiMalloc = ::mimalloc::MiMalloc;

#[ cfg (all (feature = "hss-jemalloc", feature = "hss-mimalloc", not (feature = "features-fuzzing"))) ]
compile_error! ("enable only one of `jemalloc` or `mimalloc`");




mod dependencies {
	
	#![ allow (unused_imports) ]
	
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
	
	
	#[ cfg (feature = "futures") ]
	pub use ::futures;
	
	#[ cfg (feature = "path-tree") ]
	pub use ::path_tree;
	
	#[ cfg (feature = "argparse") ]
	pub use ::argparse;
	
	#[ cfg (feature = "cpuprofiler") ]
	pub use ::cpuprofiler;
	
	#[ cfg (feature = "jemallocator") ]
	pub use ::jemallocator;
	
	#[ cfg (feature = "jemalloc-sys") ]
	pub use ::jemalloc_sys;
	
	#[ cfg (feature = "mimalloc") ]
	pub use ::mimalloc;
}

