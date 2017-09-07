# rust lib example

```bash
cd rust/staticlib
cargo build --lib

gcc -c -o testlibc.o testlibc.c -fPIC
ar rcs libtestlibc.a testlibc.o

cd ../main_project
gcc test.c ../staticlib/target/debug/libstaticlib.a -o test -lpthread -ldl
cargo build

cargo run
./test
```
