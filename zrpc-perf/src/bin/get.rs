use async_std::sync::Arc;
use async_std::task;
use futures::prelude::*;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use zenoh::*;

static DEFAULT_MODE: &str = "peer";
static DEFAULT_SIZE: &str = "8";
static DEFAULT_INT: &str = "5";
static DEFAULT_DURATION: &str = "60";

#[derive(StructOpt, Debug)]
struct GetArgs {
    /// Config file
    #[structopt(short, long, default_value = DEFAULT_MODE)]
    mode: String,
    #[structopt(short, long)]
    peer: Option<String>,
    #[structopt(short, long, default_value = DEFAULT_SIZE)]
    size: u64,
    #[structopt(short, long, default_value = DEFAULT_INT)]
    interveal: u64,
    #[structopt(short, long, default_value = DEFAULT_DURATION)]
    duration: u64,
}

#[async_std::main]
async fn main() {
    let args = GetArgs::from_args();

    let rtts = Arc::new(AtomicU64::new(0));
    let count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

    let properties = match args.peer {
        Some(peer) => format!("mode={};peer={}", args.mode, peer),
        None => format!("mode={}", args.mode),
    };
    let zproperties = Properties::from(properties);
    let zenoh = Zenoh::new(zproperties.into()).await.unwrap();
    let ws = zenoh.workspace(None).await.unwrap();

    let selector = Selector::try_from(format!("/test/{}", args.size)).unwrap();

    let kind = if args.mode == "peer" {
        "PR-GET"
    } else {
        "CR-GET"
    };

    let path = format!("/test/{}", args.size);
    let data = vec![0; args.size as usize];
    ws.put(&path.try_into().unwrap(), data.into())
        .await
        .unwrap();

    println!("MSGS,SIZE,THR,INTERVEAL,RTT_US,KIND");
    let c = count.clone();
    let s = args.size;
    let i = args.interveal;
    let rt = rtts.clone();
    task::spawn(async move {
        loop {
            task::sleep(Duration::from_secs(i)).await;
            let n = c.swap(0, Ordering::AcqRel);
            let r = rt.swap(0, Ordering::AcqRel);
            let msgs = n / i;
            let thr = (n * s * 8) / i;
            let rtt = if n == 0 { 0 } else { r / n };
            println!("{},{},{},{},{},{}", msgs, s, thr, i, rtt, kind);
        }
    });

    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(args.duration) {
        let now_q = Instant::now();
        let mut data_stream = ws.get(&selector).await.unwrap();
        while data_stream.next().await.is_some() {}
        count.fetch_add(1, Ordering::AcqRel);
        rtts.fetch_add(now_q.elapsed().as_micros() as u64, Ordering::AcqRel);
    }

    zenoh.close().await.unwrap();
}
