use std::io::Error;

trait Frontier {
    fn add_site() -> Result<(), Error>;
    fn get_next_site() -> Option<String>;
}

struct FIFOFrontier {
}

impl Frontier for FIFOFrontier {
    fn add_site() -> Result<(), Error> {
        unimplemented!();
    }

    fn get_next_site() -> Option<String> {
        unimplemented!();
    }
}
