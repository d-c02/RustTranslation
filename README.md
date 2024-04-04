# RustTranslation
## Determining difficulty of translating C to memory-safe Rust.

This repository is part of a project focusing on creating a software tool that can determine the difficulty of converting C code to Rust in a memory-safe way. The examples within the repository show how to convert certain C functions to Rust.

GeoIP.c has certain functions taken from https://github.com/borisz/geoip-api-c/blob/master/libGeoIP/GeoIP.c in the GeoIP repository.

'GeoIP_addr_to_num()' is a simple function that takes a string representing an IP from the command line and returns an integer representation of that IP. For example, 'GeoIP_addr_to_num("192.168.1.10")' = 3232235786.

In GeoIP/src/lib.rs, there is a Rust version of this function that first converts the IP string to a memory safe Rust string, and then calls a helper function to perform the same operation on that string, but in Rust.

'geoip_enable_teredo()' is an example of converting a string that uses a struct to memory-safe Rust. This is also taken from GeoIP.c in the GeoIP repository, however the geoip struct has been modified to remove certain more complicated struct members that would make the conversion process more confusing, as most of the struct members are not used anyways in 'geoip_enable_teredo()'. Much like 'addr_to_num()', 

To compile GeoIP.c, just run "gcc -L./GeoIP/target/release GeoIP.c -o GeoIPc -lGeoIP". To compile the Rust code, just run "cargo build" within the /Rust directory.
