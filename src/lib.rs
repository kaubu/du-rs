use std::path::PathBuf;
use clap::{Clap, crate_version, crate_authors};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
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