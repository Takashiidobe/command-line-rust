use anyhow::Result;

use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

use command_line_rust::host::*;

fn main() -> Result<()> {
    let Config { hostname } = get_args();

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(&hostname).unwrap();

    for address in response {
        println!("{} has address {}", hostname, address);
    }

    Ok(())
}
