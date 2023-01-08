# Web Server Installation
### 1) install rustup folowwing steps in this link
https://www.rust-lang.org/tools/install
### 2) install rust nightly
```
rustup default nightly
```
> you may need to restart your shell or run `source "$HOME/.cargo/env"` firstly
### 3) clone this repo
```
git clone https://github.com/MaKsY2/secret_santa.git
cd secret_santa/spbstu_ss/
```
### 4) Write your Postgres db url to .env file
```
echo 'DATABASE_URL=postgres://postgres:postgres@localhost/diesel_demo' > .env
```
### 5) Apply migrations
```
diesel migration run
```
### 6) Build and run
```
cargo build --package spbstu_ss --bin spbstu_ss
cargo run --package spbstu_ss --bin spbstu_ss
```
If everything has been done correctly, development server should be running on localhost:8000.<br>
##### It is highly recommended to use production server with nginx (or another server) proxy
