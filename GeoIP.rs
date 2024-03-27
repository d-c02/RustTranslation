fn main() {
	let ip_address = "192.168.1.10";
	let ip_num = geoip_addr_to_num(ip_address);
	println!("IP address {} is represented as {}", ip_address, ip_num);
}

fn geoip_addr_to_num(addr: &str) -> u32
{
	let mut octet: u32 = 0;
	let mut t: u32 = 0;
	let mut ipnum: u32 = 0;
	let mut i: i32 = 3;
	for c in addr.chars()
	{
		if c == '.'
		{
			if octet > 255
			{
				return 0;
			}
			ipnum = ipnum << 8;
			ipnum += octet;
			i -= 1;
			octet = 0;
		}
		else
		{
			t = octet;
			octet = octet << 3;
			octet += t;
			octet += t;
			if c.to_digit(10).unwrap() > 57
			{
				return 0;
			}
			octet = octet + c.to_digit(10).unwrap();
		}	
	}
	
	if octet > 255 || i != 0 {
		return 0;
	}
	ipnum = ipnum << 8;
	return ipnum + octet;
}
