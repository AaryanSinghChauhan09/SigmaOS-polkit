use ::polkit::{polkit, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = polkit::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.stop().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-polkit operational");
    Ok(())
}
