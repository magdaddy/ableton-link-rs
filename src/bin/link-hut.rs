extern crate ableton_link;

use ableton_link::{Link};
use std::time::Duration;
use std::thread::sleep;

fn main()
{
    let mut link = Link::new(120.0);
    let clock = link.clock();
    loop
    {
        link.with_app_session_state(|mut session_state|
        {
            let now = clock.micros();
            println!("now: {:?}", now);
            sleep(Duration::from_millis(100));
            session_state.set_tempo(122.0, 0);
            session_state.commit();
        });
    }
}
