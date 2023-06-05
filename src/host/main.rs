use anyhow::Result;

use trust_dns_resolver::Resolver;

use command_line_rust::host::*;

fn main() -> Result<()> {
    let Config { hostname } = get_args();

    let resolver = Resolver::from_system_conf().unwrap();

    let response = resolver.lookup_ip(&hostname).unwrap();

    for address in response {
        println!("{} has address {}", hostname, address);
    }

    Ok(())
}
