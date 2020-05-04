use url::form_urlencoded::Serializer;

use std::convert::TryFrom;

use crate::source::{BoxSource, Source, Token, TokenResponse};

#[derive(Debug)]
pub struct Metadata {
    account: &'static str,
    scopes: Vec<String>,
}

impl Metadata {
    fn new(scopes: impl Into<Vec<String>>) -> Self {
        Self {
            account: "default",
            scopes: scopes.into(),
        }
    }

    fn uri_suffix(&self) -> String {
        let query = if self.scopes.is_empty() {
            String::new()
        } else {
            Serializer::new(String::new())
                .append_pair("scopes", &self.scopes.join(","))
                .finish()
        };
        format!("instance/service-accounts/{}/token?{}", self.account, query)
    }
}

impl From<Metadata> for BoxSource {
    fn from(v: Metadata) -> Self {
        Box::new(v)
    }
}

impl Source for Metadata {
    fn token(&self) -> crate::Result<Token> {
        if !gcemeta::on_gce() {
            panic!("must be running on Google Compute Engine.")
        }
        match gcemeta::get(&self.uri_suffix())? {
            Some(resp) => {
                let resp = TokenResponse::try_from(resp.as_ref())?;
                Token::try_from(resp)
            }
            None => Err(crate::ErrorKind::TokenData.into()),
        }
    }
}

pub fn from_metadata(scopes: &[String]) -> crate::Result<Option<Metadata>> {
    if gcemeta::on_gce() {
        Ok(Some(Metadata::new(scopes)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metadata_uri_suffix() {
        let m = Metadata::new(Vec::new());
        assert_eq!(m.uri_suffix(), "instance/service-accounts/default/token?");

        let m = Metadata::new(vec![
            "https://www.googleapis.com/auth/cloud-platform".into(),
        ]);
        assert_eq!(
            m.uri_suffix(),
            "instance/service-accounts/default/token?scopes=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcloud-platform"
        );

        let m = Metadata::new(vec!["scope1".into(), "scope2".into()]);
        assert_eq!(
            m.uri_suffix(),
            "instance/service-accounts/default/token?scopes=scope1%2Cscope2",
        );
    }
}
