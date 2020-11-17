//! A simple command-line client to test the MQTT library.
#![deny(warnings)]

use futures_util::{
    stream::{
        futures_unordered::FuturesUnordered,
        StreamExt,
    },
};
#[allow(unused_imports)]
use log::{trace, debug, error, info};
use mqtt_async_client::{
    client::{
        Client,
        KeepAlive,
        Publish as PublishOpts,
        QoS,
        Subscribe as SubscribeOpts,
        SubscribeTopic,
    },
    Error,
    Result,
};
//no tls use rustls;
//no tls use std::io::Cursor;
use structopt::StructOpt;
use tokio::time::Duration;
//no tls use webpki_roots;

#[derive(Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Args {
    #[structopt(subcommand)]
    cmd: Command,

    /// Username to authenticate with, optional.
    #[structopt(long)]
    username: Option<String>,

    /// Password to authenticate with, optional.
    #[structopt(long)]
    password: Option<String>,

    /// Host to connect to, REQUIRED.
    #[structopt(long)]
    host: String,

    /// TCP/IP port to connect to.
    #[structopt(long, default_value="1883")]
    port: u16,

    /// Client ID to identify as, optional.
    #[structopt(long)]
    client_id: Option<String>,

    /// Enable TLS and set the path to a PEM file containing the
    /// CA certificate that signs the remote server's certificate.
    #[structopt(long)]
    tls_server_ca_file: Option<String>,

    /// Enable TLS and trust the CA certificates in the webpki-roots
    /// crate, ultimately Mozilla's root certificates.
    #[structopt(long)]
    tls_mozilla_root_cas: bool,

    /// Keepalive interval in seconds
    #[structopt(long, default_value("30"))]
    keep_alive: u16,

    /// Operation timeout in seconds
    #[structopt(long, default_value("20"))]
    op_timeout: u16,

    #[structopt(long, default_value("true"), possible_values(&["true", "false"]))]
    auto_connect: String,
}

#[derive(Clone, Debug, StructOpt)]
enum Command {
    Publish(Publish),
    Subscribe(Subscribe),
}

#[derive(Clone, Debug, StructOpt)]
struct Publish {
    /// Topic name to publish to. REQUIRED
    topic: String,

    /// Message payload to publish. REQUIRED.
    message: String,

    /// Quality of service code to use
    #[structopt(long,
                possible_values(&["0", "1", "2"]),
                default_value("0"))]
    qos: u8,

    /// Send multiple copies of the message.
    #[structopt(long,
                default_value("1"))]
    repeats: u32,

    #[structopt(long)]
    retain: bool,
}

#[derive(Clone, Debug, StructOpt)]
struct Subscribe {
    /// Topic names to subscribe to. REQUIRED
    topic: Vec<String>,

    #[structopt(long,
                possible_values(&["0", "1", "2"]),
                default_value("0"))]
    qos:u8,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::from_args();
    if cfg!(feature = "unsafe-logging") {
        debug!("Args: {:#?}", args);
    }
    let res = match args.cmd {
        Command::Publish(ref sub_args) => publish(sub_args.clone(), args.clone()).await,
        Command::Subscribe(ref sub_args) => subscribe(sub_args.clone(), args).await,
    };
    if let Err(e) = res {
        error!("{:?}", e);
    }
}

async fn publish(pub_args: Publish, args: Args) -> Result<()> {
    let mut client = client_from_args(args)?;
    client.connect().await?;
    let mut p = PublishOpts::new(pub_args.topic.clone(), pub_args.message.as_bytes().to_vec());
    p.set_qos(int_to_qos(pub_args.qos));
    p.set_retain(pub_args.retain);
    let futs = (0..(pub_args.repeats)).map(|_| {
        client.publish(&p)
    });
    let futs: FuturesUnordered<_> = futs.collect();
    let results_fut = futs.collect::<Vec<Result<()>>>();
    for res in results_fut.await {
        if let Err(e) = res {
            error!("Error publishing: {}", e);
        }
    }
    info!("Published topic={}, message={}", pub_args.topic, pub_args.message);
    client.disconnect().await?;
    Ok(())
}

async fn subscribe(sub_args: Subscribe, args: Args) -> Result<()> {
    let mut client = client_from_args(args)?;
    if sub_args.topic.len() == 0 {
        return Err(Error::from("You must subscribe to at least one topic."));
    }
    client.connect().await?;
    let subopts = SubscribeOpts::new(sub_args.topic.iter().map(|t|
        SubscribeTopic { qos: int_to_qos(sub_args.qos), topic_path: t.clone() }
    ).collect());
    let subres = client.subscribe(subopts).await?;
    subres.any_failures()?;
    loop {
        let r = client.read_subscriptions().await;
        info!("Read r={:?}", r);
        if let Err(Error::Disconnected) = r {
            return Err(Error::Disconnected);
        }
    }
}

fn client_from_args(args: Args) -> Result<Client> {
    let mut b = Client::builder();
    b.set_host(args.host)
     .set_port(args.port)
     .set_username(args.username)
     .set_password(args.password.map(|s| s.as_bytes().to_vec()))
     .set_client_id(args.client_id)
     .set_connect_retry_delay(Duration::from_secs(1))
     .set_keep_alive(KeepAlive::from_secs(args.keep_alive))
     .set_operation_timeout(Duration::from_secs(args.op_timeout as u64))
     .set_automatic_connect(match args.auto_connect.as_str() {
         "true" => true,
         "false" => false,
         _ => panic!("Bad validation"),
     });

/* no tls
    if let Some(s) = args.tls_server_ca_file {
        let mut cc = rustls::ClientConfig::new();
        let cert_bytes = std::fs::read(s)?;
        let cert = rustls::internal::pemfile::certs(&mut Cursor::new(&cert_bytes[..]))
            .map_err(|_| Error::from("Error parsing cert file"))?[0].clone();
        cc.root_store.add(&cert)
            .map_err(|e| Error::from_std_err(e))?;
        b.set_tls_client_config(cc);
    } else if args.tls_mozilla_root_cas {
        let mut cc = rustls::ClientConfig::new();
        cc.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        b.set_tls_client_config(cc);
    }
*/
    b.build()
}

fn int_to_qos(qos: u8) -> QoS {
    match qos {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        2 => QoS::ExactlyOnce,
        _ => panic!("Not reached"),
    }
}
