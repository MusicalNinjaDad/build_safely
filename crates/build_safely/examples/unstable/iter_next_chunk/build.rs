use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(iter_next_chunk, &allowed_features);
    Ok(())
}
