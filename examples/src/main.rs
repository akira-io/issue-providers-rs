use issue_provider_core::{IssueResult, provider};
use issue_provider_linear::linear;

fn main() -> IssueResult<()> {
    let registry = provider().register(linear())?.build();

    for descriptor in registry.descriptors() {
        println!(
            "{} ({})",
            descriptor.display_name(),
            descriptor.id().as_str()
        );
    }

    Ok(())
}
