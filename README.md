# seeip

Rust Wrapper for the IP Info <https://seeip.org/> API
## Examples
All of the below examples support both IPv4 and IPv6 addresses.
If you specifically want to use IPv4 or IPv6 use their `_v4()` and `_v6()` equivalents.
#### Get the caller's IP address
```rust
let my_ip = get_ip();
println!(my_ip);
```
#### Get the caller's geographical information
```rust
let my_geo_info = get_caller_geo();
// Fields in GeoInfo can default out to empty values if not available from the API
println!(my_geo_info.country);
```
#### Get geographical information for an IP address
```rust
let geo_info = geo_ip("208.67.222.222");
println!(geo_info.country);
```
