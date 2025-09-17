use std::ops::Index;


pub async fn all_or_nothing<T, F, FUT>(
    func: F,
) -> T
where 
    FUT: Future<Output = Result<T, Box<dyn std::error::Error>>>,
    F: Fn() -> FUT,
{
    let mut res = func().await;
    while res.is_err() {
        res = func().await;
    }
    res.unwrap()
}

pub async fn one_time<'a, T, O, F, FUT>(
    func: F,
) -> T
where 
    for<'c> &'c T: IntoIterator<Item = &'c O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
    F: Fn() -> FUT,
    FUT: Future<Output = T>,
{
    let mut res = func().await;
    let mut first = &res[0];
    while res
        .into_iter()
        .any(|v| v != first)
    {
        res = func().await;
        first = &res[0];
    }
    res
}

pub async fn one_time_hm<'a, H, T, O, F, FUT>(
    func: F,
) -> H
where
    F: Fn() -> FUT,
    FUT: Future<Output = H>,
    for<'b> &'b H: IntoIterator<Item = (&'b &'a str, &'b T)>,
    for<'b> &'b T: IntoIterator<Item = &'b O>,
    T: Index<usize, Output = O>,
    O: PartialEq,
{
    let mut res = func().await;
    let mut first = &res
        .into_iter()
        .next()
        .unwrap()
        .1
        [0];
    while res
        .into_iter()
        .any(|v| &v.1[0] != first)
    {
        res = func().await;
        first = &res
            .into_iter()
            .next()
            .unwrap()
            .1
            [0];
    }
    res
}
