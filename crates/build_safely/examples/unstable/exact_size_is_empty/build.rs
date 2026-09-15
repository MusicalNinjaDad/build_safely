use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(exact_size_is_empty, &allowed_features);
    Ok(())
}
