use std::rc::Rc;

mod arguments;
mod extensions;
mod config;
mod log;
mod message;
mod metadata;
mod repo;
mod util;

use arguments::*;
use config::Configuration;
use extensions::*;
use log::*;
use message::*;
use repo::*;
use util::*;

fn main() {
	// input parameters.
	let arg = match Arguments::collect() {
		| Ok(a) => a,
		| Err(e) => {
			grc_err_println(e.message());
			return;
		}
	};
	// parse configuration file to Extensions struct.
	let ext = if arg.has_specified_config() {
		Extensions::from(arg.config_file())
	} else {
		Extensions::from_agreement()
	}
	
	let extensions = match ext {
		| Ok(e) => e,
		| Err(e) => {
			grc_err_println(e.to_string());
			return;
		}
	};

	let config = Configuration::merge(arg, extensions);
	
	// extends types.
	let mut types: Vec<String> = vec![];
	
	let path = current_path();
	// repository Object instance.
	let repo = match Repository::new(path, Rc::clone(&arg)) {
		| Ok(r) => r,
		| Err(e) => {
			grc_err_println(e.message());
			return;
		}
	};

	// commit message.
	let message = Messager::new(Rc::clone(&arg)).load_ext_td(&types).ask().build();
	grc_println(&message);

	// Git commit
	if let Err(e) = repo.commit(message.as_str()) {
		grc_err_println(e.message());
	}
}
