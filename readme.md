# rust_add

#### This crate shows how a rust library can be used to generate a DLL that can be called by g++

### Project structure
The root project for this is the C++ project `c_rust_call` and the project tree is relative to this directory. Here is the relevant project directories.
```
c_rust_call
    ├───Inc
    ├───lib
    └───rust_add
        ├───src
        └───target
            └───release
```
The root C++ repo is stored here: [c_rust_call](http://aus-srv-dev2.lumen.com.au/paul.easter/c_rust_call) and the rust repo is stored as a git submodule here [rust_add](http://aus-srv-dev2.lumen.com.au/paul.easter/rust_add).

After installing rust, create a library with:

`cargo new rust_add --lib`

This creates a rust library:
`src/lib.rs`
which has been used, as is, with a few minor changes:

`#[no_mangle] ` has been added to keep the correct function names
`pub extern "C"` has been added to the start of any external functions

This is the entirety of the rust code, excluding the automatically generated unit test:
```rust
#[no_mangle] 
pub extern "C" fn add(left: i8, right: i8) -> i8 {
    println!("left = {}, right={}, left + right = {}",left, right,left + right);
    left + right
}
```

### To generate the initial header file for C++:
First run:
`cargo install cbindgen` in the rust project directory, and then run:
`cbindgen . -o target/rust_add.h` to generate the initial header file.

This header file will need to be copied across to the C++ project and modified (refer to `c_rust_call.cpp`).

### Modifying the header file and rust config file to generate a DLL
Add the following to `rust_add.h` if you want to generate a dynamic library (DLL) windows.

`#define rust_add_API __declspec(dllimport)`

This needs to correspond to the following lines in `Config.toml` in the Rust project:
```rust
[lib]
crate-type=["cdylib"]
```
### Modify the C++ header file to import the rust function
Add the following lines so that the C++ compiler is aware of the external function.

```Cpp
extern "C" {

    rust_add_API int8_t add(int8_t left, int8_t right);

} // extern "C" 
```
### C++ Makefile
Finally, the make file has been modified to build both the rust and C++ project using `g++`. This make file performs a clean build each time but this behaviour can be modified as desired. The compiler used is packaged in [Msys2](https://www.msys2.org/) and is installed in a post-installation step by executing:
`pacman -S mingw-w64-x86_64-gcc`
Also `C:\msys64\mingw64\bin` should be added into your Windows path.

Here is the full make file:

```makefile
CC = g++ -g -Wall

INC = ./Inc
RUSTDIR = ./rust_add
DLLTARGETDIR = ./lib
DLLSOURCEDIR = $(RUSTDIR)/target/release
LIBS = -L$(DLLTARGETDIR)/ -lrust_add

generated=c_rust_call.exe c_rust_call.o $(DLLTARGETDIR)/rust_add.dll


all: clean buildrust copydll c_rust_call.exe 

c_rust_call.exe: c_rust_call.o c_rust_call.cpp $(DLLTARGETDIR)/rust_add.dll
	$(CC) -o c_rust_call.exe c_rust_call.o $(LIBS)

c_rust_call.o: c_rust_call.cpp
	$(CC) -c c_rust_call.cpp 

clean:
	rm -f $(generated)

buildrust:
	cargo clean --manifest-path $(RUSTDIR)/Cargo.toml
	cargo build --release --manifest-path $(RUSTDIR)/Cargo.toml

copydll:
	cp $(DLLSOURCEDIR)/rust_add.dll $(DLLTARGETDIR)
```



