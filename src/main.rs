// Copyright 2022 - 2023 Wenmeng See the COPYRIGHT
// file at the top-level directory of this distribution.
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
// 
// Author: tickbh
// -----
// Created Date: 2023/08/16 06:51:59

// #![deny(warnings)]
use wmproxy::{ProxyResult, ControlServer, Helper, arg};

async fn run_main() -> ProxyResult<()> {
    let option = arg::parse_env().await?;
    Helper::try_init_log(&option);
    let control = ControlServer::new(option);
    control.start_serve().await?;
    Ok(())
}

// #[forever_rs::main]
#[tokio::main]
async fn main() {
    if let Err(e) = run_main().await {
        println!("运行wmproxy发生错误:{:?}", e);
    }
}
