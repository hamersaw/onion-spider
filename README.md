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

##Test Sites
xmh57jrzrnw6insl - Torch Search Engine

##TODO
- change lib::execute_polzat_crawl(_) to use link_extractor::extract_map(_)
- stats command
- perhaps add an allow_regex for robots.txt parsing?
- tor crawl
- scrape
