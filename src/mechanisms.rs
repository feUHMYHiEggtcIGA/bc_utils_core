use std::ops::Index;
use std::time::{
    SystemTime,
    UNIX_EPOCH
};
use std::error::Error;


pub async fn all_or_nothing<T, F, FUT>(
    func: F,
    wait_time_sec: &f64,
) -> Result<T, Box<dyn Error>>
where 
    FUT: Future<Output = Result<T, Box<dyn std::error::Error>>>,
    F: Fn() -> FUT,
{
    let mut res = func().await;
    let timenow = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
    while {
        res.is_err()
        && *wait_time_sec != f64::INFINITY
        && *wait_time_sec > SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64() - timenow
    } {
        res = func().await;
    }
    res
}

pub async fn one_time<'a, T, O, F, FUT>(
    func: F,
) -> Result<T, Box<dyn Error>>
where 
    for<'c> &'c T: IntoIterator<Item = &'c O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
    F: Fn() -> FUT,
    FUT: Future<Output = Result<T, Box<dyn Error>>>,
{
    let mut res = func().await?;
    let mut first = res.into_iter().next().ok_or(Box::<dyn Error>::from("res first err in one time"))?;
    while res
        .into_iter()
        .any(|v| v != first)
    {
        res = func().await?;
        first = res.into_iter().next().ok_or(Box::<dyn Error>::from("res first err in one time cycle"))?;
    }
    Ok(res)
}

pub async fn one_time_hm<'a, H, T, O, F, FUT>(
    func: F,
) -> Result<H, Box<dyn Error>>
where
    F: Fn() -> FUT,
    FUT: Future<Output =Result <H, Box<dyn Error>>>,
    for<'b> &'b H: IntoIterator<Item = (&'b &'a str, &'b T)>,
    for<'b> &'b T: IntoIterator<Item = &'b O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
{
    let mut res = func().await?;
    let mut first = &res
        .into_iter()
        .next()
        .ok_or(Box::<dyn Error>::from("first err"))
        ?
        .1
        [0];
    while res
        .into_iter()
        .any(|v| &v.1[0] != first)
    {
        res = func().await?;
        first = &res
            .into_iter()
            .next()
            .ok_or(Box::<dyn Error>::from("first err"))
            ?
            .1
            [0];
    }
    Ok(res)
}
