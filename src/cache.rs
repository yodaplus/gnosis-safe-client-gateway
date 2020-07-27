use rocket_contrib::databases::redis::{self, Commands};
use serde_json;
use serde::ser::{Serialize};
use anyhow::Result;
use rocket::response::content;

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

impl ServiceCache {
    pub fn fetch(&self, id: &String) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            Err(_e) => None,
        }
    }

    pub fn create(&self, id: &String, dest: &String, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    pub fn invalidate(&self, id: &String) {
        let _: () = self.del(id).unwrap();
    }

    pub fn cache_resp<S, R>(&self, key: &String, timeout: usize, resp: S) -> Result<content::Json<String>>
    where 
        S: Fn() -> Result<R>,
        R: Serialize
    {
        let cached = self.fetch(key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp = resp()?;
                let resp_string = serde_json::to_string(&resp)?;
                self.create(key, &resp_string, timeout);
                Ok(content::Json(resp_string))
            },
        }
    }
}