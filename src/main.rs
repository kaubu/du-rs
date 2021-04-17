use clap::{Clap, crate_version, crate_authors};
use humansize::{self, FileSize, file_size_opts};
use std::{
	error::Error,
	io,
	path::{Path, PathBuf},
	env,
	fs
};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
	/// Directory to start from (default = current directory)
	pub dir: Option<PathBuf>,
	#[clap(short, long )]
	//// Show size in a human-reable way
	pub human_readable: bool,
	#[clap(short, long )]
	/// Produce a summary for the directory
	pub summarize: bool,
	#[clap(short = 'l', long)]
	/// Count sizes many times if hard links
	pub count_links: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
	let opts = Opts::parse();

	let start_dir = match opts.dir {
		Some(dir) => dir,
		_ => env::current_dir()?,
	};

	let usage = calc_space_usage(start_dir.clone())?;
	let human_usage = usage.file_size(file_size_opts::CONVENTIONAL)?;

	println!("{}\t{}", human_usage, start_dir.display());
	Ok(())
}

fn calc_space_usage(path: PathBuf) -> io::Result<u64> {
	let mut paths = vec![path];
	let mut size = 0;

	while let Some(path) = paths.pop() {
		let meta = fs::symlink_metadata(&path)?;
		let file_type = meta.file_type();

		if file_type.is_symlink() {
			// Do nothing
		} else if file_type.is_dir() {
			let entries = fs::read_dir(path)?;

			for entry in entries {
				let entry = entry?;
				paths.push(entry.path());
			}
		} else if file_type.is_file() {
			size += meta.len();
		}
	}

	Ok(size)
}