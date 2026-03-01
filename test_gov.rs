use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;

fn main() {
    let lim = RateLimiter::direct(Quota::per_second(NonZeroU32::new(10).unwrap()));
    let res = lim.check_n(NonZeroU32::new(1).unwrap());
    let _: () = res;
}
