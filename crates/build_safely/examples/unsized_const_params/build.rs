use autocfg::AutoCfg;
use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(unsized_const_params, &allowed_features);
    Ok(())
}
