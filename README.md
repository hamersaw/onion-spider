#polzat

##Overview
Web crawler/scraper written in rust. Purpose focuses on academia and 
analysis on differential treatment of anonymous users.

##Requirements
- tor
- torsocks

##Components
####polzatd
This is the daemon application. Currently it is configured through command line
arguments using the docopt crate. Once finished it will be configured with a 
toml file.

####polzat
The client application. Configured through command line arguements using the 
docopt crate. Has two operations, namely crawl and stats. Crawl is issued with 
a list of onion hidden servies to crawl. Stats requires no arguments.

##Test Sites
xmh57jrzrnw6insl - Torch Search Engine

##TODO
- look at robots.txt
- tor crawl
- scrape
