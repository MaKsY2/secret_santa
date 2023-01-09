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

# Available endpoints
>POST /login

## Users
>GET /users

>GET /users/<id>

>POST /users/<id> (used for signup)
>```json
>{
>  "name": String,
>  "password": String
>}
>```

>PUT /users/<id> (login required)
>```json
>{
>  "name": String
>}
>```

>DELETE /users/<id> (login required)
## Groups
>GET /groups

>GET /groups/<id>

>POST /groups (login required)
>```json
>{
>  "name": String
>}
>```

>PUT /groups/<id> (admin role required)
>```json
>{
>  "name": String
>}
>```

> DELETE /groups/<id> (admin role required)
## Memberships (controls if user is in group)
>GET /memberships?<user_id>&<group_id>

>POST /memberships?<group_id> (login required)
> ```json
>{
> "group_id": integer,
> "user_id": integer
>}
>```

>PUT /memberships?<group_id>&<user_id> (admin role required, used for admins to give admin permission)
>```json
>{
>  "role": "admin/member"
>}
>```
>DELETE /memberships?<group_id>&<user_id> (login required)

## Santas (controls santas assignment)
>GET /santas?<group_id>&<user_id> (login required)

>POST /santas?<group_id> (admin role required, used to randomly assign santa to each person in group)
