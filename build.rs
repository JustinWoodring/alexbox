use url::Url;
use std::process::Command;
use std::env;
use std::path::Path;
use std::fs::canonicalize;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let unc_path = canonicalize(env::current_dir().unwrap().join("src").join("clientapp")).expect("canonicalization error");
	let file_url = Url::from_file_path(unc_path).expect("url");
	let abs_path_buf = file_url.to_file_path().expect("path");
	let abs_path = abs_path_buf.as_path();
    // Tell Cargo that if the given file changes, to rerun this build script.
    //println!("cargo:rerun-if-changed=src/clientapp");
	// Use the `cc` crate to build a C file and statically link it.
	println!("cargo:rerun-if-changed=src/clientapp");
	println!("cargo:rerun-if-changed=Cargo.lock");
	println!("{}",abs_path.to_str().unwrap());
	let output = if cfg!(target_os = "windows") {
		println!("{}", &("WINDOWS STATIC NPM BUILD"));
		Command::new("cmd")
				.current_dir(abs_path)
				.args(&["/C", "npm install"])
				.output()
				.expect("failed to execute process");
		Command::new("cmd")
				.current_dir(abs_path)
				.args(&["/C", "npm run build"])
				.output()
				.expect("failed to execute process");
		Command::new("cmd")
				.args(&["/C", "rmdir /q /s static_files"])
				//.args(&["/C", "rmdir", &(out_dir.clone()+"\\..\\..\\..\\static_files /q")])
				.output()
				.expect("failed to execute process");
		Command::new("cmd")
				.args(&["/C", "move src\\clientapp\\build","static_files"]) 
				//.args(&["/C", "move src\\clientapp\\build",&(out_dir.clone()+"\\..\\..\\..\\static_files")]) 
				.output()
				.expect("failed to execute process");
		println!("{}", &(out_dir+"\\static_files"));
	} else {
		println!("{}", &("LINUX STATIC NPM BUILD"));
		Command::new("sh")
				.current_dir(abs_path)
				.args(&["-c", "npm install"])
				.output()
				.expect("failed to execute process");
		Command::new("sh")
				.current_dir(abs_path)
				.args(&["-c", "npm run build"])
				.output()
				.expect("failed to execute process");
		Command::new("sh")
				.args(&["-c", "rm -r -f static_files"])
				//.args(&["-c", "rm -r", &(out_dir.clone()+"\\..\\..\\..\\static_files")])
				.output()
				.expect("failed to execute process");
		Command::new("sh")
				.args(&["-c", "mv src/clientapp/build static_files"]) 
				//.args(&["-c", "mv src/clientapp/build",&(out_dir.clone()+"/../../../static_files")]) 
				.output()
				.expect("failed to execute process");
	};
}
