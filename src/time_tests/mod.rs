use time;
use time::{Duration, PreciseTime, SteadyTime, Timespec};
use std::cmp::Ordering;

pub fn execute () {
    assert_eq!(Duration::weeks(1).cmp(&Duration::days(3)),Ordering::Greater);
    assert_eq!(Duration::hours(1).cmp(&Duration::minutes(70)),Ordering::Less);
    assert_eq!(Duration::seconds(1).cmp(&Duration::milliseconds(1000)),Ordering::Equal);
    assert_eq!(Duration::microseconds(1).cmp(&Duration::nanoseconds(1)),Ordering::Greater);

    let d = Duration::span(|| {
        let big_slice   = &[1u8; 10000];
        let mut _sum:u64 = 0;
        for value in big_slice.iter() {
            _sum += *value as u64;
        }
    });

    println!("Duration: {}µs",d.num_microseconds().unwrap());

    assert_eq!(Duration::hours(5).checked_add(&Duration::hours(20)).unwrap(),Duration::hours(25));
    assert_eq!(Duration::minutes(1).checked_sub(&Duration::seconds(30)).unwrap(),Duration::seconds(30));
    assert!(Duration::zero().is_zero());

    let start_time = PreciseTime::now();
    while start_time.to( PreciseTime::now() ) < Duration::milliseconds(1) {
        println!("hello");
    }
    let start_time = SteadyTime::now();
    while SteadyTime::now() - start_time < Duration::milliseconds(1) {
        println!("goodbye");
    }
    assert_eq!(Timespec::new(100,0) - Timespec::new(50,0),Duration::seconds(50));

    let ts:Timespec = time::now().to_timespec();
    println!("current epoch: {}",ts.sec);
    println!("current local time: {}",time::at(ts).strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("current UTC time: {}",time::at_utc(ts).strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("empty time: {:?}",time::empty_tm());
    println!("current epoch: {}",time::get_time().sec);
    println!("current UTC time: {}",time::now_utc().strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("another strftime: {}",time::strftime("%Y-%m-%d %H:%M:%S",&time::now()).unwrap());

    let dates:Vec<&str>        = vec!["1981-07-24","09/23/1983","04-27-1987"];
    let date_formats:Vec<&str> = vec!["%Y-%m-%d","%m/%d/%Y","%m-%d-%Y"];
    for date in &dates {
        for format in &date_formats {
            match time::strptime(date,format) {
                Ok(time) => {
                    println!("date({}) as rfc3339: {}",date,time.rfc3339());
                    break;
                }
                Err(_) => continue,
            }
        }
    }
    println!("time since unspecified epoch: {}",time::precise_time_s());
    println!("time since unspecified epoch as i64: {}",time::precise_time_s() as i64);
    println!("unspecified epoch???: {}",(time::now() - Duration::seconds(time::precise_time_s() as i64)).rfc3339());
    println!("current ctime: {}",time::now().ctime());
}
