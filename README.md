# seeip

Rust Wrapper for the IP Info <https://seeip.org/> API
## Examples
All of the below examples support both IPv4 and IPv6 addresses.  
To specifically use either IPv4 or IPv6 use their `_v4()` and `_v6()` equivalents.
#### Get the caller's IP address
```rust
let my_ip = seeip::get_ip().unwrap();
println!("My IP: {}", my_ip);
```
#### Get the caller's geographical information
```rust
let my_geo_info = seeip::get_caller_geo().unwrap();
// Fields in GeoInfo can default out to empty values if not available from the API
println!("Country my IP matches: {}", my_geo_info.country);
```
#### Get geographical information for an IP address
```rust
let geo_info = seeip::get_geo("208.67.222.222").unwrap();
println!("Country matching this IP: {}", geo_info.country);
```

## Testing
The tests involve making actual API calls. This means that they can fail if you make an IPv6-only call when your host does not support IPv6:

If you are on a host supporting both IPv4 and IPv6: `cargo test --verbose`  
For a host that only supports IPv4: `cargo test --verbose -- --skip v6`  
For a host that only supports IPv6: `cargo test --verbose -- --skip v4`  
