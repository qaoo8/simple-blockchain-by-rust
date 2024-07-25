mod block;
mod errors;
mod cli;
mod blockchain;

use errors::Result;

fn main() -> Result<()> {
    let mut cli = cli::Cli::new()?;
    cli.run()?;
    Ok(())
}
