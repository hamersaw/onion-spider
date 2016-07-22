#onion-spider

##Overview
Web spider for TOR written in rust. Consists of a daemon application and a
corresponding client application. Can schedule crawls with the option to make 
it recursive through the client applicataion. Client application is also able
to retreive elementary statistics from daemon appliction. Future work is to 
implement a distributed hash table to allow distributed crawls.

##Requirements
- tor
- torsocks
- wget

##Components
####onion-spider
This is the daemon application. Currently it is configured through command line
arguments using the docopt crate. Once finished it will be configured with a 
toml file.

####yogi
The client application. Configured through command line arguements using the 
docopt crate. Has two operations, namely crawl and stats. Crawl is issued with 
a list of onion hidden servies to crawl. Stats requires no arguments.

##Test Sites
xmh57jrzrnw6insl - Torch Search Engine

##TODO
- add recursive flag to client application
- exclude image files from website fetch
- implement link extracting - close to a working crawler
