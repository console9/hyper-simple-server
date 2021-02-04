

#![ no_implicit_prelude ]




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
#![ warn (unreachable_pub) ]
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




pub(crate) mod accepter;
pub(crate) mod configuration;
pub(crate) mod connection;
pub(crate) mod errors;
pub(crate) mod exports;
pub(crate) mod handler;
pub(crate) mod main;
pub(crate) mod prelude;
pub(crate) mod routes;
pub(crate) mod server;




pub use {
	
	crate::accepter::*,
	crate::connection::*,
	crate::configuration::*,
	crate::errors::*,
	crate::exports::*,
	crate::handler::*,
	crate::routes::*,
	crate::server::*,
	
	crate::main::main,
};




pub use ::hyper;
pub use ::tokio;
pub use ::http_body;

pub use ::rustls;
pub use ::tokio_rustls;
pub use ::rustls_pemfile;

pub use ::native_tls;
pub use ::tokio_native_tls;

#[ allow (unused_imports) ]
pub(crate) use ::futures;
#[ allow (unused_imports) ]
pub(crate) use ::path_tree;

