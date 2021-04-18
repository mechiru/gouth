use crate::Token;
use tonic::{metadata::MetadataValue, Interceptor, Request, Status};

pub fn interceptor() -> impl Into<Interceptor> {
    let token = Token::new().expect("Token::new()");
    move |mut req: Request<()>| {
        let token = &*token
            .header_value()
            .map_err(|e| Status::unknown(e.to_string()))?;
        let meta = MetadataValue::from_str(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    }
}
