use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = "Usage")]
pub struct Opt {
  #[structopt(help = "Input file")]
  pub input: String,
  #[structopt(help = "Column Name")]
  pub column_name: String,
  #[structopt(help = "Output file, stdio if not present")]
  pub output: Option<String>,
}
