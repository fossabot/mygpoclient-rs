use crate::Client;
use crate::Error;
use reqwest::IntoUrl;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A Subscription as returned by [`Client::get_all_subscriptions`]
#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    pub url: String,
    pub title: String,
    pub description: String,
    pub subscribers: u16,
    pub subscribers_last_week: u16,
    pub logo_url: Option<String>,
    pub scaled_logo_url: Option<String>,
    pub website: Option<String>,
    pub mygpo_link: String,
}

impl Client {
    /// Get All Subscriptions
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-all-subscriptions
    pub fn get_all_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        get_internal(
            &format!("https://gpodder.net/subscriptions/{}.json", self.username),
            &self.username,
            &self.password,
        )
    }

    /// Get Subscriptions of Device
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#get-subscriptions-of-device
    pub fn get_subscriptions(&self, deviceid: &str) -> Result<Vec<String>, Error> {
        get_internal(
            &format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.username, deviceid
            ),
            &self.username,
            &self.password,
        )
    }

    /// Upload Subscriptions of Device
    ///
    /// # See also
    /// https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html#upload-subscriptions-of-device
    pub fn put_subscriptions(&self, subscriptions: &[String], deviceid: &str) -> Result<(), Error> {
        reqwest::Client::new()
            .put(&format!(
                "https://gpodder.net/subscriptions/{}/{}.json",
                self.username, deviceid
            ))
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(subscriptions)
            .send()?; // TODO handle response?
        Ok(())
    }
}

fn get_internal<U: IntoUrl, T: DeserializeOwned>(
    url: U,
    username: &str,
    password: &str,
) -> Result<T, Error> {
    Ok(reqwest::Client::new()
        .get(url)
        .basic_auth(username, Some(password))
        .header(
            reqwest::header::USER_AGENT,
            &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
        )
        .send()?
        .json()?) // TODO handle response?
}

// TODO unit tests
