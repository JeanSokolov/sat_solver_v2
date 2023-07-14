# KI
## Rust
### Compiler ("cargo") aufsetzen
Official installation instructions: https://doc.rust-lang.org/cargo/getting-started/installation.html

Direct download-link for Windows: https://win.rustup.rs/

For Windows: 
  
  Download rustup-init.exe
  
  Execute rustup-init.exe
  
  Follow instructions of the CLI and regard the following:
  
  Some Rust code/crates(libraries) utilize/import C code as Rust is C-compatible. Therefore the system needs to have a C compiler installed.
    
  During installation you will be prompted with a standard installation, which includes the (for windows) default target-triple of x86_64-pc-windows-msvc. If you prefer using GNU C compilers you can change this setting to x86_64-pc-windows-gnu.
    
  This setting can be changed at anytime and can also be set to cross-compilation targets like aarch64-unknown-linux-gnu or the like with the command rustup target add [target-triple] (possible targets can be listed with rustup target list). At compilation you then need to include the target in the build instruction e.g. cargo build --target aarch64-unknown-linux-gnu --release.

  If you want to compile your project normally you can simply run the command cargo build. Adding the flag -r or --release results in building the release version of the project.
  

### Simplex-algorithm
Source code at src/main.rs

Binary at x86_64-pc-windows-gnu/sat_solver_v2.exe

Current slides: Implementierung eines Simplex-Algorithmus gekürzt.pdf

