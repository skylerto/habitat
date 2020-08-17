use crate::{common::ui::{UIWriter,
                         UI},
            error::Result};
use habitat_core::{crypto::keys::{generate_service_encryption_key_pair,
                                  Key,
                                  KeyCache},
                   service::ServiceGroup};

pub fn start(ui: &mut UI,
             org: &str,
             service_group: &ServiceGroup,
             key_cache: &KeyCache)
             -> Result<()> {
    ui.begin(format!("Generating service key for {} in {}", &service_group, org))?;
    let (public, secret) = generate_service_encryption_key_pair(org, &service_group.to_string());
    key_cache.write_key(&public)?;
    key_cache.write_key(&secret)?;
    ui.end(format!("Generated service key pair {}.", &public.named_revision()))?;
    Ok(())
}
