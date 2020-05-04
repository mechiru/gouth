pub mod google {
    pub mod pubsub {
        pub mod v1 {
            tonic::include_proto!("google.pubsub.v1");
        }
    }
}

use google::pubsub::v1::{publisher_client::PublisherClient, ListTopicsRequest};

use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

use gouth::Token;

const ENDPOINT: &str = "https://pubsub.googleapis.com";

// Read the file downloaded from https://pki.goog/roots.pem.
// See https://cloud.google.com/blog/products/gcp/google-cloud-services-are-switching-certificate-authority.
const CERTIFICATES: &[u8] = include_bytes!("./../../data/roots.pem");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let token = Token::new()?;

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("pubsub.googleapis.com");

    let channel = Channel::from_static(ENDPOINT)
        .tls_config(tls_config)
        .connect()
        .await?;

    let mut service = PublisherClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().unwrap();
        let meta = MetadataValue::from_str(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });

    for i in 0..u64::MAX {
        let response = service
            .list_topics(Request::new(ListTopicsRequest {
                project: format!("projects/{}", project),
                page_size: 10,
                ..Default::default()
            }))
            .await?;

        tokio::time::delay_for(std::time::Duration::from_secs(5)).await;

        println!(
            "duration={:?}, RESPONSE={:?}",
            std::time::Duration::from_secs(i * 5),
            response
        );
    }

    Ok(())
}
