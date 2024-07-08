use std::env;
use std::net::{Ipv4Addr, AddrParseError};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

// Define a custom error type for network parsing errors
#[derive(Debug)]
enum NetworkParseError {
    AddrParseError(AddrParseError),
    IntParseError(ParseIntError),
    InvalidFormat,
    InvalidMask,
}

impl fmt::Display for NetworkParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkParseError::AddrParseError(e) => write!(f, "Address parse error: {}", e),
            NetworkParseError::IntParseError(e) => write!(f, "Integer parse error: {}", e),
            NetworkParseError::InvalidFormat => write!(f, "Invalid input format"),
            NetworkParseError::InvalidMask => write!(f, "Invalid subnet mask"),
        }
    }
}

// Implement conversion from standard errors to custom errors
impl From<AddrParseError> for NetworkParseError {
    fn from(err: AddrParseError) -> NetworkParseError {
        NetworkParseError::AddrParseError(err)
    }
}

impl From<ParseIntError> for NetworkParseError {
    fn from(err: ParseIntError) -> NetworkParseError {
        NetworkParseError::IntParseError(err)
    }
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments is provided
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: {} <network/mask or network mask>", args[0]);
        std::process::exit(1);
    }

    // Parse arguments and handle errors
    match parse_args(&args) {
        Ok((network, mask)) => {
            let network_address = Ipv4Addr::from(u32::from(network) & u32::from(mask));
            let broadcast_address = Ipv4Addr::from(u32::from(network) | !u32::from(mask));
            let first_address = if mask_to_prefix_length(mask) == 32 { network_address } else { Ipv4Addr::from(u32::from(network_address) + 1) };
            let last_address = if mask_to_prefix_length(mask) == 32 { network_address } else { Ipv4Addr::from(u32::from(broadcast_address) - 1) };
            let wildcard_mask = Ipv4Addr::from(!u32::from(mask));
            let host_count = if mask_to_prefix_length(mask) == 32 { 1 } else { (u32::from(broadcast_address) - u32::from(network_address) - 1) as usize };
            let network_class = get_network_class(network);
            let prefix_length = mask_to_prefix_length(mask);

            // Print out the network details
            println!("\x1b[0m{:<10} {:<30} {}", "Address:", network, to_colored_binary_string(network, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "Netmask:", format!("{} = {}", mask, prefix_length), to_colored_binary_string(mask, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "Wildcard:", wildcard_mask, to_colored_binary_string(wildcard_mask, prefix_length));
            println!("=>");
            println!("\x1b[0m{:<10} {:<30} {}", "Network:", format!("{}/{}", network_address, prefix_length), to_colored_binary_string(network_address, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "HostMin:", first_address, to_colored_binary_string(first_address, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "HostMax:", last_address, to_colored_binary_string(last_address, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "Broadcast:", broadcast_address, to_colored_binary_string(broadcast_address, prefix_length));
            println!("\x1b[0m{:<10} {:<30} {}", "Hosts/Net:", host_count, format!("Class {}, {}", network_class, get_network_type(network)));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

// Parse command-line arguments
fn parse_args(args: &[String]) -> Result<(Ipv4Addr, Ipv4Addr), NetworkParseError> {
    if args.len() == 2 {
        let input = &args[1];
        if input.contains('/') {
            let parts: Vec<&str> = input.split('/').collect();
            if parts.len() != 2 {
                return Err(NetworkParseError::InvalidFormat);
            }
            let network = Ipv4Addr::from_str(parts[0])?;
            let prefix_length = parts[1].parse::<u32>()?;
            if prefix_length > 32 {
                return Err(NetworkParseError::InvalidMask);
            }
            let mask = !0u32 << (32 - prefix_length);
            return Ok((network, Ipv4Addr::from(mask)));
        } else {
            return Err(NetworkParseError::InvalidFormat);
        }
    } else if args.len() == 3 {
        let network = Ipv4Addr::from_str(&args[1])?;
        let mask = parse_mask(&args[2])?;
        return Ok((network, mask));
    } else {
        Err(NetworkParseError::InvalidFormat)
    }
}

// Parse the subnet mask, either in dotted decimal or prefix length format
fn parse_mask(mask: &str) -> Result<Ipv4Addr, NetworkParseError> {
    if mask.contains('.') {
        Ipv4Addr::from_str(mask).map_err(NetworkParseError::from)
    } else {
        let prefix_length = mask.parse::<u32>()?;
        if prefix_length > 32 {
            return Err(NetworkParseError::InvalidMask);
        }
        let mask = !0u32 << (32 - prefix_length);
        Ok(Ipv4Addr::from(mask))
    }
}

// Convert a subnet mask to its prefix length
fn mask_to_prefix_length(mask: Ipv4Addr) -> u32 {
    u32::from(mask).count_ones()
}

// Convert an IP address to a colored binary string with correct dot placement
fn to_colored_binary_string(addr: Ipv4Addr, prefix_length: u32) -> String {
    // Create the binary string without dots initially
    let binary_string = format!(
        "{:08b}{:08b}{:08b}{:08b}",
        addr.octets()[0],
        addr.octets()[1],
        addr.octets()[2],
        addr.octets()[3]
    );

    // Ensure proper dot placement
    let binary_string_with_dots = add_dots(&binary_string);

    // Convert prefix_length to usize
    let split_index = (prefix_length + prefix_length / 8) as usize;

    // Split the binary string at the prefix length
    let (network_part, host_part) = binary_string_with_dots.split_at(split_index);

    // Colorize the network part
    let colored_network_part = network_part
        .chars()
        .map(|c| if c == '.' { c.to_string() } else { format!("\x1b[32m{}\x1b[0m", c) })
        .collect::<String>();

    // Colorize the host part
    let colored_host_part = host_part
        .chars()
        .map(|c| if c == '.' { c.to_string() } else { format!("\x1b[31m{}\x1b[0m", c) })
        .collect::<String>();

    // Combine the colored network and host parts
    format!("{}{}", colored_network_part, colored_host_part)
}

// Add dots every 8 bits to the binary string
fn add_dots(binary_string: &str) -> String {
    binary_string
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            acc.push(c);
            if (i + 1) % 8 == 0 && i != binary_string.len() - 1 {
                acc.push('.');
            }
            acc
        })
}

// Determine the class of a network based on the first octet of the IP address
fn get_network_class(ip: Ipv4Addr) -> char {
    let octet = ip.octets()[0];
    match octet {
        0..=127 => 'A',
        128..=191 => 'B',
        192..=223 => 'C',
        224..=239 => 'D',
        _ => 'E',
    }
}

// Determine if the network is private or public based on the IP address
fn get_network_type(ip: Ipv4Addr) -> &'static str {
    let octets = ip.octets();
    match octets[0] {
        10 => "Private Internet",
        172 if (16..=31).contains(&octets[1]) => "Private Internet",
        192 if octets[1] == 168 => "Private Internet",
        _ => "Public Internet",
    }
}