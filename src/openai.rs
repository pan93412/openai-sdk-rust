//! OpenAI: The entry of OpenAI that can be used
//! to call API methods.

mod features;

use cached::proc_macro::cached;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, AUTHORIZATION},
    Url,
};

#[derive(Default)]
/// A high-level method for building a [`OpenAI`] client
pub struct OpenAIBuilder<'a> {
    token: Option<&'a str>,
    org: Option<&'a str>,
}

/// The OpenAI client with the specified token and organization.
///
/// Note that it includes **NO** high-level methods. You *must* include
/// the *features* trait in [`crate::features`].
pub struct OpenAI {
    // [TODO) allow customized client
    pub client: reqwest::Client,
}

impl<'a> OpenAIBuilder<'a> {
    /// Create a builder for filling the token and organization.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the token for the OpenAI client.
    pub fn token(&mut self, token: &'a str) -> &mut Self {
        self.token = Some(token);
        self
    }

    /// Set the organization for the OpenAI client.
    pub fn organization(&mut self, org: &'a str) -> &mut Self {
        self.org = Some(org);
        self
    }

    /// Build the OpenAI client.
    ///
    /// It will check if the token is provided.
    /// If `organization` is provided, it will be also filled in.
    pub fn build(&mut self) -> Result<OpenAI, BuilderError> {
        const USER_AGENT: &str = concat!("openai-rs/", env!("CARGO_PKG_VERSION"));

        let mut header = HeaderMap::with_capacity(1);

        if let Some(token) = self.token.take() {
            let token = HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(BuilderError::AuthHeader)?;

            header.append(AUTHORIZATION, token);
        } else {
            return Err(BuilderError::NoToken);
        }

        if let Some(org) = self.org.take() {
            let org = HeaderValue::from_str(org).map_err(BuilderError::OrgHeader)?;

            header.append(HeaderName::from_static("openai-organization"), org);
        }

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(header)
            .build()?;

        Ok(OpenAI::new(client))
    }
}

impl OpenAI {
    /// Create a OpenAI client.
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

/// Create an standard endpoint URL to call the OpenAI API.
///
/// - `version`: the version of the API, e.g. `v1`
/// - `endpoint`: the endpoint of the API, e.g. `engines`
#[cached(key = "String", convert = r#"{ format!("{}/{}", version, endpoint) }"#)]
pub fn openai_uri(version: &str, endpoint: &str) -> Result<Url, url::ParseError> {
    #[cfg(debug_assertions)]
    {
        assert!(version.starts_with('v'), "version must start with 'v'");
    }

    let mut component = String::with_capacity(version.len() + endpoint.len() + 1);
    component.push_str(version);
    component.push('/');
    component.push_str(endpoint);

    let url = Url::parse("https://api.openai.com/")?.join(&component)?;

    Ok(url)
}

/// The error of [`OpenAIBuilder`].
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("no OpenAI token provided")]
    NoToken,

    #[error("authorization header: {0}")]
    AuthHeader(InvalidHeaderValue),

    #[error("organization header: {0}")]
    OrgHeader(InvalidHeaderValue),

    #[error("client error: {0}")]
    Client(#[from] reqwest::Error),
}

/// The error of [`OpenAI`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("url: {0}")]
    Url(#[from] url::ParseError),

    #[error("client: {0}")]
    Client(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    mod builder {
        use super::super::{BuilderError, OpenAIBuilder};

        #[test]
        fn test_new() {
            let builder = OpenAIBuilder::new();

            assert!(builder.token.is_none());
            assert!(builder.org.is_none());
        }

        #[test]
        fn test_token_set() {
            let mut builder = OpenAIBuilder::new();

            builder.token("token");

            assert!(builder.token.is_some());
            assert!(builder.org.is_none());
        }

        #[test]
        fn test_org_set() {
            let mut builder = OpenAIBuilder::new();

            builder.organization("org");

            assert!(builder.token.is_none());
            assert!(builder.org.is_some());
        }

        #[test]
        fn test_build_no_token() {
            let mut builder = OpenAIBuilder::new();

            assert!(matches!(builder.build(), Err(BuilderError::NoToken)));
        }

        #[test]
        fn test_build_token_invaild() {
            let mut builder = OpenAIBuilder::new();

            builder.token("\x1b[0b");

            assert!(matches!(builder.build(), Err(BuilderError::AuthHeader(_))));
        }

        #[test]
        fn test_build_token_vaild() {
            let mut builder = OpenAIBuilder::new();

            builder.token("hello");

            assert!(builder.build().is_ok());
        }

        #[test]
        fn test_build_org_invaild() {
            let mut builder = OpenAIBuilder::new();

            builder.token("hello");
            builder.organization("\x1b[0b");

            assert!(matches!(builder.build(), Err(BuilderError::OrgHeader(_))));
        }

        #[test]
        fn test_build_token_org_vaild() {
            let mut builder = OpenAIBuilder::new();

            builder.token("hello");
            builder.organization("org");

            assert!(builder.build().is_ok());
        }
    }

    mod uri {
        use crate::openai::openai_uri;

        #[test]
        fn test_v1_uri_valid() {
            let testtable = [
                ("v1", "engines", "https://api.openai.com/v1/engines"),
                (
                    "v1",
                    "engines/davinci",
                    "https://api.openai.com/v1/engines/davinci",
                ),
                (
                    "v1",
                    "engines/davinci/completions",
                    "https://api.openai.com/v1/engines/davinci/completions",
                ),
            ];

            for (version, endpoint, expected) in testtable.into_iter() {
                assert_eq!(openai_uri(version, endpoint).unwrap().to_string(), expected);
            }
        }

        #[test]
        #[should_panic]
        fn test_v1_uri_invalid() {
            openai_uri("1", "engines/davinci/").unwrap();
        }
    }
}
