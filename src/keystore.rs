#![allow(non_snake_case)]

use serde_derive;

use serde_json::{self, Value};
use request::Handler;
use error::ConsulResult;
use std::error::Error;

pub struct Keystore{
    handler: Handler
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct KVPair {
    #[serde(rename="Key")]
    pub key: String,
    #[serde(rename="CreateIndex")]
    pub create_index: u64,
    #[serde(rename="Value")]
    pub value: Option<String>
}

impl Keystore {
    pub fn new(address: &str) ->  Keystore {
        Keystore {
            handler: Handler::new(&format!("{}/v1/kv", address))
        }
    }

    pub fn set_key(&self, key: String, value: String) -> ConsulResult<()> {
        self.handler.put(&key, value, Some("application/json"))?;
        Ok(())
    }

    pub fn acquire_lock(&self, key: String, address: String, session_id: &String) -> ConsulResult<bool> {
        let uri = format!("{}?acquire={}", key, session_id);
        let result = self.handler.put(&uri, address, Some("application/json"))?;
        if result == "true" {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn release_lock(&self, key: String, address: &str, session_id: &String) -> ConsulResult<bool> {
        let uri = format!("{}?release={}", key, session_id);
        let result = self.handler.put(&uri, address.to_owned(), Some("application/json"))?;
        if result == "true" {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn get_key(&self, key: String) -> ConsulResult<Option<String>> {
        let result = self.handler.get(&key)?;
        let json_data: Value = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;
        let v_json = json_data.as_array().unwrap();
        Ok(super::get_string(&v_json[0], &["Value"]))
    }

    pub fn get_kvpair(&self, key: String) -> ConsulResult<Option<KVPair>> {
        let result = self.handler.get(&key)?;
        let kv_pairs: Vec<KVPair> = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;

        if kv_pairs.len() > 0 {
            return Ok(Some(kv_pairs[0].clone()));
        }
        Ok(None)
    }

    pub fn list(&self, key: String) -> ConsulResult<Option<Vec<KVPair>>> {
        let key_with_params = format!("{}?recurse", key.trim_right_matches('/'));
        let result = self.handler.get(&key_with_params)?;

        let kv_pairs: Vec<KVPair> = serde_json::from_str(&result)
            .map_err(|e| e.description().to_owned())?;

        Ok(Some(kv_pairs))
    }

    pub fn delete_key(&self, key: String) {
        self.handler.delete(&key).unwrap();
    }
}
