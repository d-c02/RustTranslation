use core::ffi::c_char;
use core::ffi::CStr;

#[repr(C)]
pub struct geoip {
	pub file_path: *const cty::c_char,
	pub cache: *const cty::c_uchar,
	pub index_cache: *const cty::c_uchar,
	pub database_segments: *const cty::c_uint,
	pub database_type: cty::c_char,
	pub mtime: cty::int64_t,
	pub flags: cty::c_int,
	pub size: cty::uint64_t,
	pub record_length: cty::c_char,
	pub charset: cty::c_int,
	pub record_iter: cty::c_int,
	pub netmask: cty::c_int,
	pub last_mtime_check: cty::int64_t,
	pub dyn_seg_size: cty::uint64_t,
	pub ext_flags: cty::c_uint,
}

pub struct newgeoip {
	pub file_path: String,
	pub cache: String,
	pub index_cache: String,
	pub database_segments: String,
	pub database_type: char,
	pub mtime: i64,
	pub flags: i32,
	pub size: u64,
	pub record_length: char,
	pub charset: i32,
	pub record_iter: i32,
	pub netmask: i32,
	pub last_mtime_check: i64,
	pub dyn_seg_size: u64,
	pub ext_flags: u32,
}

fn geoip_to_new_geoip(gi: *mut geoip) -> newgeoip
{
	let mut gi2: newgeoip;
	unsafe{
		
		gi2.file_path = CStr::from_ptr((*gi).file_path).to_str().expect("REASON").to_string();
		gi2.cache = CStr::from_ptr((*gi).cache as *const i8).to_str().expect("REASON").to_string();
		gi2.index_cache = CStr::from_ptr((*gi).index_cache as *const i8).to_str().expect("REASON").to_string();
		gi2.database_segments = CStr::from_ptr((*gi).database_segments as *const i8).to_str().expect("REASON").to_string();
		gi2.database_type = char::from_u32((*gi).database_type as u32).unwrap();
		gi2.mtime = (*gi).mtime;
		gi2.flags = (*gi).flags;
		gi2.size = (*gi).size;
		gi2.record_length = char::from_u32((*gi).record_length as u32).unwrap();
		gi2.charset = (*gi).charset;
		gi2.record_iter = (*gi).record_iter;
		gi2.netmask = (*gi).netmask;
		gi2.last_mtime_check = (*gi).last_mtime_check;
		gi2.dyn_seg_size = (*gi).dyn_seg_size;
		gi2.ext_flags = (*gi).ext_flags;
		}
	
	return gi2
}

#[no_mangle]
pub extern "C" fn geoip_enable_teredo(gi: *mut geoip, true_false: u32) -> u32
{
	let mut tmp: i32;
	let mut mask: u32 = 1 << 0;
	let mut b: u32 = 0;
	unsafe
	{
	if (*gi).ext_flags == mask
	{
		b = 1;
	}
	else
	{
		b = 0;
	}
	}
	unsafe
	{
	(*gi).ext_flags &= !mask;
	}
	if true_false != 0
	{
		unsafe
		{
		(*gi).ext_flags |= true_false;
		}
	}
	print!("Called Rust GeoIP!\n");
	return b;
}

#[no_mangle]
pub extern "C" fn geoip_addr_to_num(ipstr: *const c_char) -> u32
{
	let ipcstr: &CStr = unsafe {CStr::from_ptr(ipstr)};
	print!("Called Rust GeoIP!\n");
	return geoip_addr_to_num_helper(ipcstr.to_str().expect("REASON"));
}

#[no_mangle]
pub extern "C" fn geoip_addr_to_num_helper(addr: &str) -> u32
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
