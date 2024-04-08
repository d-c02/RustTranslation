# RustTranslation

## Overview
Our goal is to generate tools which simplify transitioning C code to safe Rust.
To do so, we would like to identify patterns in original C code which are easier to translate to safe Rust.
As a first step, we would like to manually look into sample C code and determine whether they can be easily translated to safe Rust, by a beginner Rust developer.
The first library we have considered is libGeoIP. We chose this library because it is both used in large programs (e.g., Nginx) and its code is easy to understand.
We plan on extending our manual analysis to other libraries as well.

## LibGeoIP
TODO: What is libgeoIP? How many functions does it have?

We have chosen two functions to analyze: `X` and `Y`. In the following section we will discuss each function separately.

### Build libgeoIP with and without Rust
We would like to translate part of libGeoIP to Rust and confirm that the functionality is not affected.
To do so ...

### Function `X`
The first function we looked into is: `X`
This function performs ... and is a leaf function (i.e., does not call any other function)

### Function `Y`
The second function we looked into is `Y`.
This function performs ...

GeoIP.c has certain functions taken from https://github.com/borisz/geoip-api-c/blob/master/libGeoIP/GeoIP.c in the GeoIP repository.

'GeoIP_addr_to_num()' is a simple function that takes a string representing an IP from the command line and returns an integer representation of that IP. For example, 'GeoIP_addr_to_num("192.168.1.10")' = 3232235786.

In GeoIP/src/lib.rs, there is a Rust version of this function that first converts the IP string to a memory safe Rust string, and then calls a helper function to perform the same operation on that string, but in Rust.

'geoip_enable_teredo()' is an example of converting a string that uses a struct to memory-safe Rust. This is also taken from GeoIP.c in the GeoIP repository, however the geoip struct has been modified to remove certain more complicated struct members that would make the conversion process more confusing, as most of the struct members are not used anyways in 'geoip_enable_teredo()'. Much like 'addr_to_num()', 

To compile GeoIP.c, just run "gcc -L./GeoIP/target/release GeoIP.c -o GeoIPc -lGeoIP". To compile the Rust code, just run "cargo build" within the /Rust directory.
