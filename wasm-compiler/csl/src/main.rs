use cli_args;

fn main() {
  let cli = cli_args::parse_args();
  println!("name: {:?}", cli.file_name.as_deref());
  println!("output: {:?}", cli.output.as_deref());
}
