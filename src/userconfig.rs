
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde_json;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig{
    pub token: String,
    pub deviceName: String,
    pub location: String,
    pub serverURL: String,
}


pub fn saveConfig(clientConfig: ClientConfig){
        let j = serde_json::to_string(&clientConfig);

        match j {
            Ok(v) => {
                    let path = Path::new("/data/addify.config");
                    let display = path.display();

                    //Open a file in write-only mode, returns `io::Result<File>`
                    let mut file = match File::create(&path) {
                        Err(why) => panic!("couldn't create {}: {}",
                                        display,
                                        why.description()),
                        Ok(mut f) => {
                            match f.write_all(v.as_bytes()) {
                                Err(why) => {
                                    panic!("couldn't write to {}: {}", display,
                                                                    why.description())
                                },
                                Ok(_) => println!("successfully wrote to {}", display),
                            }
                        }
                    };
            },
            Err(e) => println!("error {:?}", e),
        }
}

pub fn restartAddifyService(){

    let output = Command::new("systemctl")
                     .arg(" restart addify.service")
                     .output()
                     .expect("Failed to execute command");
}