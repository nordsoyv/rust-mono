use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub file_name: Option<String>,

    #[arg(short,long)]
    pub output : Option<String>
}

pub fn parse_args() -> Cli {
    let cli = Cli::parse();
    cli
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
