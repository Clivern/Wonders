// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use postgres::{Client, NoTls};
use std::process;

use crate::util;

pub fn tables() {
    let rocket_config =
        util::config::get_env("ROCKET_CONFIG", util::config::get_config_path().as_str());
    let config = util::config::get_configs(rocket_config.to_string());
    let client = Client::connect(config.global.db.as_str(), NoTls);

    match client.expect("failed to migrate").batch_execute(
        "
        CREATE  TABLE IF NOT EXISTS wo_post (
            id                   integer  NOT NULL,
            name                 varchar(100)  NOT NULL,
            payload              text  NOT NULL,
            status               varchar(100)  NOT NULL,
            created_at           timestamp  NOT NULL,
            updated_at           timestamp  NOT NULL,
            CONSTRAINT pk_wo_post PRIMARY KEY ( id )
         );
    ",
    ) {
        Ok(_) => {
            println!("Database migrated successfully!");
        }
        Err(err) => {
            println!("Error while migration: {:?}", err);
            process::exit(1);
        }
    }
}
