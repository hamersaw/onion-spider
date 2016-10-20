#polzat

##Overview
Web crawler/scraper written in rust. Purpose focuses on academia and 
analysis on differential treatment of anonymous users.

##Components
####polzatd
A daemon application. Currently configuration is manifested by command 
line arguments, although plans are set for toml configuration.

####polzat
A client application configured through command line arguments. Currently
one operation is implemented, namely 'crawl'.

##Compiling
protoc --rust_out=src/ protobuf/*.proto
protoc --rust-grpc_out=src/ protobuf/*.proto
cargo build

##Test Sites
xmh57jrzrnw6insl - Torch Search Engine

##TODO
- solve read lock on frontier (shouldn't be locked)
- url validator for TOR urls
- perhaps add an allow_regex for robots.txt parsing?
- tor crawl
- scrape
