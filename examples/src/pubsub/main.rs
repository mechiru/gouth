use googapis::{
    google::pubsub::v1::{publisher_client::PublisherClient, ListTopicsRequest},
    CERTIFICATES,
};
use gouth::tonic::interceptor;
use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("pubsub.googleapis.com");

    let channel = Channel::from_static("https://pubsub.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut service = PublisherClient::with_interceptor(channel, interceptor());

    for i in 0..u64::MAX {
        let response = service
            .list_topics(Request::new(ListTopicsRequest {
                project: format!("projects/{}", project),
                page_size: 10,
                ..Default::default()
            }))
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        println!(
            "duration={:?}, RESPONSE={:?}",
            std::time::Duration::from_secs(i * 5),
            response
        );
    }

    Ok(())
}
