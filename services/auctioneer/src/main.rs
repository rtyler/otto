/**!
 * The auctioneer main
 */
#[deny(unsafe_code)]
extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
}

#[cfg(test)]
mod tests {}
