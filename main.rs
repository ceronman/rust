pub trait MyTryFrom<T> {
    type Error;
    fn my_try_from(value: T) -> u8;
}

pub trait MyInto<T> {}

impl<T, U> MyTryFrom<U> for T
    where U: MyInto<T>,
{
    type Error = i8;

    fn my_try_from(_: U) -> u8 {
        0
    }
}

impl MyTryFrom<i32> for u64 {
    type Error = u8;

    fn my_try_from(_: i32) -> u8 {
        0
    }
}

fn main() {
    let _ = u64::my_try_from(0i32);
    println!("END");
}