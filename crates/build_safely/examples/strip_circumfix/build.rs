use autocfg::AutoCfg;
use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(strip_circumfix, &allowed_features);
    Ok(())
}
