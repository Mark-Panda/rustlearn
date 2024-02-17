


pub type DynHttpClientExt = Arc<dyn HttpClientExt + Send + Sync>;
use std::{sync::Arc, time::Duration};
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Response;
use serde::Serialize;


#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: reqwest::Client,
}

impl HttpClient  {
    pub async fn connect(timeout: u64) -> Result<Self> {
        let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()?;
        Ok(Self { client })
    }
}



#[async_trait]
pub trait HttpClientExt {
  async fn post_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error>;
  async fn put_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error>;
  async fn delete_request(&self, url: &str) -> Result<Response, reqwest::Error>;
  async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error>;
}


#[async_trait]
impl HttpClientExt for HttpClient {
  async fn post_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error> {
    self.client.post(url).json(body).send().await
  }

  async fn put_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error> {
    self.client.put(url).json(body).send().await
  }

  async fn delete_request(&self, url: &str) -> Result<Response, reqwest::Error> {
    self.client.delete(url).send().await
  }

  async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error> {
    self.client.get(url).send().await
  }
}
