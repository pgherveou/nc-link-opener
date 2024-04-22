use std::env::args;

use auto_launch::AutoLaunchBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(port) = args().nth(1) else {
        panic!("Usage: nc-link-opener-launcher <port>");
    };

    let port: u16 = port.parse()?;
    let auto = AutoLaunchBuilder::new()
        .set_app_name("nc-link-opener")
        .set_app_path("/usr/local/bin/nc-link-opener")
        .set_args(&[port.to_string()])
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    auto.enable()?;
    auto.is_enabled()?;
    Ok(())
}
