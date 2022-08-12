

#[ allow (unused_imports) ]
use crate::prelude::*;




#[ cfg (feature = "cpuprofiler") ]
pub struct ProfilingSession {
	path_final : path::PathBuf,
	path_temporary : path::PathBuf,
	active : usize,
}


#[ cfg (feature = "cpuprofiler") ]
impl ProfilingSession {
	
	
	pub fn new (_path : &path::Path) -> StdIoResult<Self> {
		
		let _path_final = _path.to_owned ();
		
		let _path_temporary = {
			let mut _path_temporary = _path_final.clone () .into_os_string ();
			_path_temporary.push (format! (".{}.tmp", process::id ()));
			_path_temporary.into ()
		};
		
		let _self = Self {
				path_final : _path_final,
				path_temporary : _path_temporary,
				active : 0,
			};
		
		Ok (_self)
	}
	
	
	pub fn new_and_start (_path : &path::Path) -> StdIoResult<Self> {
		let mut _self = Self::new (_path) ?;
		_self.start () ?;
		Ok (_self)
	}
	
	
	pub fn start (&mut self) -> StdIoResult {
		
		if self.active == 0 {
			profiling_start (&self.path_temporary) ?;
		}
		
		self.active += 1;
		
		Ok (())
	}
	
	
	pub fn stop (&mut self) -> StdIoResult {
		
		if self.active == 0 {
			return Err (error_with_code (0x628def32));
		}
		
		self.active -= 1;
		
		if self.active == 0 {
			
			profiling_stop () ?;
			
			fs::rename (&self.path_temporary, &self.path_final) .else_wrap (0xfc22794e) ?;
		}
		
		Ok (())
	}
	
	
	pub fn stop_and_drop (mut self) -> StdIoResult {
		self.drop_0 ()
	}
	
	fn drop_0 (&mut self) -> StdIoResult {
		if self.active == 0 {
			return Ok (());
		}
		self.active = 1;
		return self.stop ();
	}
}


#[ cfg (feature = "cpuprofiler") ]
impl Drop for ProfilingSession {
	
	fn drop (&mut self) -> () {
		self.drop_0 () .else_panic (0xaebddcc0);
	}
}




#[ cfg (feature = "cpuprofiler") ]
fn profiling_start (_path : &path::Path) -> StdIoResult {
	
	let _path = _path.to_str () .else_wrap (0x977d8538) ?;
	let _path = _path.to_owned () .into_bytes ();
	
	#[ cfg (debug_assertions) ]
	eprintln! ("[ii] [1c05ae71]  starting `cpuprofiler` tracing...");
	
	let mut _profiler = ::cpuprofiler::PROFILER.lock () .else_wrap (0xd30eee91) ?;
	
	if _profiler.state () != ::cpuprofiler::ProfilerState::NotActive {
		return Err (error_with_code (0x1bd8ceb5));
	}
	
	_profiler.start (_path) .else_wrap (0x57e487d1) ?;
	
	Ok (())
}


#[ cfg (feature = "cpuprofiler") ]
fn profiling_stop () -> StdIoResult {
	
	#[ cfg (debug_assertions) ]
	eprintln! ("[ii] [27a3b301]  stopping `cpuprofiler` tracing...");
	
	let mut _profiler = ::cpuprofiler::PROFILER.lock () .else_wrap (0x678aa104) ?;
	
	if _profiler.state () != ::cpuprofiler::ProfilerState::Active {
		return Err (error_with_code (0x5dff5e52));
	}
	
	_profiler.stop () .else_wrap (0x39363dfd) ?;
	
	Ok (())
}

