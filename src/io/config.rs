use toml;
use std::io::Read;
use std::fs::File;
use std::fmt::Write;
use crypto::scrypt::{scrypt, ScryptParams};

pub struct Config {
    scrypt_params: ScryptParams,
    salt: Vec<u8>,
}

impl Config {
    pub fn load() -> Config {
        let mut data = String::new();
        let mut f = File::open("../../config.toml").expect("couldn't open config.toml");
        f.read_to_string(&mut data)
            .expect("couldn't read config.toml");
        let t = data.parse::<toml::Value>()
            .expect("couldn't parse config.toml");

        Config {
            scrypt_params: ScryptParams::new(
                t["crypto"]["log_n"]
                    .as_integer()
                    .expect("scrypt's log_n parameter was not set properly") as u8,
                t["crypto"]["r"]
                    .as_integer()
                    .expect("scrypt's r parameter was not set properly") as u32,
                t["crypto"]["p"]
                    .as_integer()
                    .expect("scrypt's p parameter was not set properly") as u32,
            ),
            salt: t["crypto"]["salt"]
                .as_array()
                .expect("scrypt's salt was not set properly")
                .iter()
                .map(|v| v.as_integer().unwrap() as u8)
                .collect()
        }
    }

    pub fn hash(&self, s: &str) -> String {
        let mut out = [0u8; 32];
        scrypt(s.as_bytes(), &*self.salt, &self.scrypt_params, &mut out);
        let mut s = String::new();
        for &byte in out.iter() {
            write!(&mut s, "{:X} ", byte).expect("Unable to write");
        }

        s
    }
}
