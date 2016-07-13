#onion-spider

##Overview
Web spider for TOR written in rust.

##Design
onion_spider
    description: client application to interact with daemon application

    commands
        crawl-site <site>
        frontier-size

onion_spider_d
    description: daemon application to crawl tor

    toml configuration file
    ip address and port of other fetchers



    Frontier
        fn get_next_site() -> Option<String>
        
    Fetcher
        fn fetch_site(site: String) -> Result<(), Error>

    LinkExtractor
        fn extract_links(site: String) -> Result<Vec<String>, Error>

##TODO
- do it all brah
