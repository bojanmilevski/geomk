use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, default_value = "database")]
	pub db_name: String,

	#[arg(short, long, default_value = "drinking_water")]
	pub query: String,

	#[arg(short, long, default_value = "coordinates")]
	pub table_name: String,
}
