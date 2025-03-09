use crate::time::{sleep_tokio, unix_time};
use reqwest::{Client, Response, header::HeaderMap};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct Request {
	pub client: Client,
	pub headers: HeaderMap,
	pub interval: Duration,
	pub interval_step: Duration,
	pub interval_max: Duration,
	pub timeout: Duration,
	pub max_tries: u32,
}

macro_rules! api_request {
	($fun:ident, $method:ident) => {
		pub async fn $fun(
			request: &Request, url: &str,
		) -> Result<Response, Box<dyn std::error::Error>> {
			let mut interval = request.interval;
			let mut tries = 1;
			loop {
				let start_time = unix_time()?;
				let res = timeout(
					request.timeout,
					request
						.client
						.$method(url)
						.headers(request.headers.clone())
						.send(),
				)
				.await;
				match res {
					Ok(Ok(response)) => {
						return Ok(response);
					}
					Ok(Err(error)) => {
						println!("Request failed: {error}");
					}
					Err(error) => {
						println!("Request timed out: {error}");
					}
				}
				sleep_tokio(start_time, interval).await?;
				interval = if interval < request.interval_max {
					interval + request.interval_step
				} else {
					request.interval_max
				};
				if tries > request.max_tries {
					return Err("Max tries reached!".into());
				}
				tries += 1;
			}
		}
	};
}

api_request!(api_get_request, get);
api_request!(api_delete_request, delete);

macro_rules! api_request_with_body {
	($fun:ident, $method:ident) => {
		pub async fn $fun(
			request: &Request, body: String, url: &str,
		) -> Result<Response, Box<dyn std::error::Error>> {
			let mut interval = request.interval;
			let mut tries = 1;
			loop {
				let start_time = unix_time()?;
				let res = timeout(
					request.timeout,
					request
						.client
						.$method(url)
						.body(body.clone())
						.headers(request.headers.clone())
						.send(),
				)
				.await;
				match res {
					Ok(Ok(response)) => {
						return Ok(response);
					}
					Ok(Err(error)) => {
						println!("Request failed: {error}");
					}
					Err(error) => {
						println!("Request timed out: {error}");
					}
				}
				sleep_tokio(start_time, interval).await?;
				interval = if interval < request.interval_max {
					interval + request.interval_step
				} else {
					request.interval_max
				};
				if tries > request.max_tries {
					return Err("Max tries reached!".into());
				}
				tries += 1;
			}
		}
	};
}

api_request_with_body!(api_put_request, put);
api_request_with_body!(api_post_request, post);
api_request_with_body!(api_patch_request, patch);
