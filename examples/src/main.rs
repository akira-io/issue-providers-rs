use omnitrack::{IssueFilter, IssueResult, Issues, provider};
use omnitrack::linear::linear;

#[tokio::main]
async fn main() -> IssueResult<()> {
    let registry = provider().register(linear())?.build();

    for descriptor in registry.descriptors() {
        println!(
            "{} ({})",
            descriptor.display_name(),
            descriptor.id().as_str()
        );
    }

    if let Ok(token) = std::env::var("LINEAR_TOKEN") {
        let client = linear().token(token).build();
        let page = client.list(IssueFilter::default(), None).await?;
        println!("fetched {} issue(s)", page.items().len());
        for item in page.items() {
            println!("- {} [{}]", item.title(), item.status());
        }
    } else {
        println!("set LINEAR_TOKEN to fetch live issues");
    }

    Ok(())
}
