// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::thread;
use std::time::Duration;

pub fn daemon() {
    let duration = Duration::from_secs(24 * 60 * 60);

    println!("Daemon started rolling!");

    while true {
        thread::sleep(duration);
        println!("24 hours have passed!");
    }
}
