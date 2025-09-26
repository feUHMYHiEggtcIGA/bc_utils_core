use std::{collections::HashMap, error::Error};
use std::collections::hash_map::RandomState;

use tokio;

use bc_utils_core::mechanisms::*;


#[tokio::test]
async fn one_time_res_1() -> Result<(), Box<dyn Error>>
{
    one_time(
        async || Ok(vec![1, 1]), 
        |v| Ok(v),
        |v| Ok(v.first().ok_or(Box::<dyn Error>::from("first el err"))?),
    ).await?;
    Ok(())
}

#[tokio::test]
async fn one_time_res_2() -> Result<(), Box<dyn Error>>
{
    one_time_hm(
        async || Ok(HashMap::<_, _, RandomState>::from_iter([
            ("1", "word"),
            ("2", "word"),
        ])),
        |v| Ok(v.1),
        |v| Ok(v.iter().next().ok_or(Box::<dyn Error>::from("first el err"))?.1)
    ).await?;
    Ok(())
}
