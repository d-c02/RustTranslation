# RustTranslation

## Overview
Our goal is to generate tools which simplify transitioning C code to safe Rust.
To do so, we would like to identify patterns in original C code which are easier to translate to safe Rust.
As a first step, we would like to manually look into sample C code and determine whether they can be easily translated to safe Rust, by a beginner Rust developer.
The first library we have considered is libGeoIP. We chose this library because it is both used in large programs (e.g., Nginx) and its code is easy to understand.
We plan on extending our manual analysis to other libraries as well.

## LibGeoIP
libGeoIP is a C library that gets location data from IPv4 and IPv6 addresses, including city name, region, country, continent, ISO codes, timezone, latitude, longitude and more. It contains 5 .c files, the largest of which, "GeoIP.c" (https://github.com/borisz/geoip-api-c/blob/master/libGeoIP/GeoIP.c), containing many functions that reference each other, global variables, dynamic allocation, and structs. As a result, converting this repository to memory-safe Rust automatically would be a complicated problem to solve.

We have chosen two functions to analyze: `GeoIP_addr_to_num()` and `geoip_enable_teredo()`. In the following section we will discuss each function separately.

### Build libgeoIP with and without Rust
We would like to translate part of libGeoIP to Rust and confirm that the functionality is not affected.
To do so, we translated these two functions, created C code to call them, and then ensured that the outputs match between calling the original C code and calling the Rust code from C.

### `GeoIP_addr_to_num("192.168.1.10")`
The first function we looked into is `GeoIP_addr_to_num().`
This function performs takes a string representing an IP from the command line and returns an integer representation of that IP. For example, `GeoIP_addr_to_num("192.168.1.10") = 3232235786`. It is also a leaf function (i.e., does not call any other function), so it can be translated more easily without having to worry about converting any other functions.

In GeoIP/src/lib.rs, there is a Rust version of this function that first converts the IP string to a memory safe Rust string, and then calls a helper function to perform the same operation on that string as `GeoIP_addr_to_num()` in GeoIP.c, but in Rust.


### Function 'geoip_enable_teredo()'
The second function we looked into is `geoip_enable_teredo()`. This function takes a GeoIP struct and a boolean, and then uses those in a bitwise operation on members of the struct and returns a true or false value. This is also a leaf function.

`geoip_enable_teredo()` is an example of converting a string that uses a struct to memory-safe Rust. This is also taken from GeoIP.c in the GeoIP repository, however the geoip struct has been modified to remove certain more complicated struct members that would make the conversion process more confusing, as most of the struct members are not used in `geoip_enable_teredo()`. Much like `GeoIP_addr_to_num()`, this function can be called from C to ensure the output still matches the same input.

### Compilation
To compile GeoIP.c, just run `gcc -L./GeoIP/target/release GeoIP.c -o GeoIPc -lGeoIP`. To compile the Rust code, just run `cargo build` within the /Rust directory.
