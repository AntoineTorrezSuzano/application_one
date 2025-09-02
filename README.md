# Project application_one
*v1.0.0-alpha*
This projet is only one half of the true project, the other half can be found here : https://github.com/AntoineTorrezSuzano/application_one_server_backend/tree/v1.0.0-alpha

It contains simply two file: `app.rs` and `main.rs`

## Core Info
This half-project was developed in a Windows 11 environement

You need to have Rust (rustc) and Cargo (it comes with Rust) installed on your device
You can install Rust here : https://www.rust-lang.org/tools/install

#### Run Debug
```
cargo run
```
#### Run Release
```
cargo run --release
```
##### Note about release
I have added a bunch of parameter in the Cargo.toml for the profile.release :
```TOML
[profile.release]
opt-level = 'z' 
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```
--- 
##### Explanation
- `opt-level = 'z'` 
- `lto = true`
These settings aggressively optimize for a small executable. `opt-level = 'z'` minimizes the size of each individual code module, while `lto = true` performs a final optimization on the entire program, which further reduces size by removing unused code and improving overall efficiency.
- `codegen-units = 1`
This setting indicate that the compilation is done my only **one** process which will slow the compile time but for a maximally optimized program in size and performance (again)
- `panic = 'abort'`
This setting tell us that when the compiled program with encounter a fatal error, it will directly stop without any waste in ressource or waiting times
- `strip = true`
This setting removes debugging symbols and other metadata that could helps reverse-engineers to understand the compiled program
