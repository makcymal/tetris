use std::{
	path::PathBuf,
	// io::Write,
	fs, env,
};


fn main() {
	let target = env::var("TARGET").unwrap();

	// let mut file = fs::File::options().create(true).write(true).open("log.txt").unwrap();
	// write!(file, "{}", target);

	if !target.contains("pc-windows") {
		return;
	}

	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

	let mut dll_dir = manifest_dir.clone();
	let mut lib_dir = manifest_dir.clone();

	if target.contains("msvc") {
		dll_dir.push("msvc");
		lib_dir.push("msvc");
	} else {
		dll_dir.push("mingw");
		lib_dir.push("mingw");
	}

	dll_dir.push("dll");
	lib_dir.push("lib");

	if target.contains("x86_64") {
		dll_dir.push("64");
		lib_dir.push("64");
	} else {
		dll_dir.push("32");
		lib_dir.push("32");
	}

	let dll_dir_iter = fs::read_dir(dll_dir).expect("Can't read DLL dir");

	for entry in dll_dir_iter {
		let entry_path = entry.expect("Invalid entry").path();

		if let Some(filename) = entry_path.file_name() {
			let filename = filename.to_str().expect("Can't read DLL name");

			if filename.ends_with(".dll") {
				let mut new_filename = manifest_dir.clone();
				new_filename.push(filename);

				fs::copy(&entry_path, new_filename.as_path()).expect("Can't copy");
			}
		}
	}
}
