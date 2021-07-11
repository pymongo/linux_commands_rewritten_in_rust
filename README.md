## cargo test must run in **single thread**

first you need to copy config file and edit it(eg. your mysql password):

> cp database_config.toml.example database_config.toml && vim database_config.toml

because multi database adapters test is using a **same file** to store data

> RUST_TEST_THREADS=1 cargo test

or 

> cargo test -- --test-threads=1

## known issue on nightly-armv7-unknown-linux-gnueabihf

- database::adapters::dbm  may double-free or malloc corrupted
