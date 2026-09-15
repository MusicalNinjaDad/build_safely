use build_safely::prelude::*;

fn main() -> Result<()> {
    let mut ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(const_ops, &allowed_features);
    Ok(())
}
