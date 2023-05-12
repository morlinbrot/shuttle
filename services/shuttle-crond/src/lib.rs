//! Shuttle service integration for cron.
use std::str::FromStr;

use async_trait::async_trait;
use chrono::Utc;
use cron::Schedule;
use serde::{Deserialize, Serialize};
use shuttle_common::resource::Type;
use shuttle_persist::PersistInstance;
use shuttle_runtime::{
    tracing::{debug, info},
    ResourceBuilder,
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    sync::oneshot,
    time::sleep,
};

mod error;
use error::CrontabServiceError;

pub type ShuttleCrond = Result<CrondService, shuttle_runtime::Error>;

type Responder<T> = oneshot::Sender<Result<T, CrontabServiceError>>;

pub type SendableJob = Box<dyn CrontabServiceJob + Send + 'static>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawJob {
    schedule: String,
    url: String,
}

pub enum Msg {
    NewJob(SendableJob, Responder<()>),
}

#[derive(Debug, Serialize, Deserialize)]
struct Crontab {
    jobs: Vec<RawJob>,
}

#[derive(Debug)]
struct CronJob {
    schedule: Schedule,
    url: String,
}

impl CronJob {
    async fn run(&self) {
        debug!("Running job for: {}", self.url);
        while let Some(next_run) = self.schedule.upcoming(Utc).next() {
            let next_run_in = next_run
                .signed_duration_since(chrono::offset::Utc::now())
                .to_std()
                .unwrap();
            sleep(next_run_in).await;

            let res = reqwest::get(self.url.clone()).await.unwrap();
            info!("Called {} with response {}", self.url, res.status());
        }
    }
}

impl From<&RawJob> for CronJob {
    fn from(raw: &RawJob) -> Self {
        let schedule = Schedule::from_str(&raw.schedule).expect("Failed to parse schedule");
        Self {
            schedule,
            url: raw.url.clone(),
        }
    }
}

struct CronRunner {
    persist: PersistInstance,
    receiver: Receiver<Msg>,
}

impl CronRunner {
    async fn run_jobs(&mut self) {
        if let Ok(tab) = self.persist.load::<Crontab>("crontab") {
            debug!("Found {} jobs", tab.jobs.len());
            for raw in tab.jobs {
                debug!("Starting job: {:?}", raw);
                let job = CronJob::from(&raw);

                tokio::spawn(async move {
                    job.run().await;
                });
            }
        } else {
            info!("Didn't find any jobs. POST to /crontab/set to create one.");
        }

        while let Some(msg) = self.receiver.recv().await {
            // let (raw, resp) = match msg {
            //     Msg::NewJob(raw, resp) => (raw, resp),
            // };
            // debug!("Channel received: {:?}", raw);

            // let mut crontab = match self.persist.load::<Crontab>("crontab") {
            //     Ok(tab) => tab,
            //     Err(_) => Crontab { jobs: vec![] },
            // };

            // let job = CronJob::from(&raw);

            // crontab.jobs.push(raw);

            // debug!("Persisting {:?} jobs", crontab.jobs.len());
            // let res = self.persist.save("crontab", crontab).map_err(From::from);
            // let _ = resp.send(res);

            // tokio::spawn(async move {
            //     job.run().await;
            // });
        }
    }
}

// struct Crond {}

// #[derive(Serialize, Deserialize)]
// struct CrondInstance {}

// impl ResourceBuilder<CrondInstance> for Crond {
//     const TYPE: Type = Type::Persist;

//     type Config = ();

//     type Output = CrondInstance;

//     fn new() -> Self {
//         todo!()
//     }

//     fn config(&self) -> &Self::Config {
//         todo!()
//     }

//     fn output<'life0, 'async_trait>(
//         self,
//         factory: &'life0 mut dyn shuttle_runtime::Factory,
//     ) -> core::pin::Pin<
//         Box<
//             dyn core::future::Future<Output = Result<Self::Output, crate::Error>>
//                 + core::marker::Send
//                 + 'async_trait,
//         >,
//     >
//     where
//         'life0: 'async_trait,
//         Self: 'async_trait,
//     {
//         todo!()
//     }

//     fn build<'life0, 'async_trait>(
//         build_data: &'life0 Self::Output,
//     ) -> core::pin::Pin<
//         Box<
//             dyn core::future::Future<Output = Result<T, crate::Error>>
//                 + core::marker::Send
//                 + 'async_trait,
//         >,
//     >
//     where
//         'life0: 'async_trait,
//         Self: 'async_trait,
//     {
//         todo!()
//     }
// }

#[async_trait]
pub trait CrontabServiceJob {
    async fn run(&mut self) -> Result<(), anyhow::Error>;

    fn schedule(&self) -> String;
}

pub struct CrondServiceBuilder {}

pub struct CrondService {
    jobs: Vec<Box<dyn CrontabServiceJob + Send>>,
    runner: CronRunner,
    sender: Sender<Msg>,
}

impl CrondService {
    pub fn new(
        persist: PersistInstance,
        jobs: Vec<Box<dyn CrontabServiceJob + Send + 'static>>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let runner = CronRunner { persist, receiver };
        Self {
            jobs,
            runner,
            sender,
        }
    }

    pub fn get_sender(&self) -> Sender<Msg> {
        self.sender.clone()
    }
}

#[async_trait]
impl shuttle_runtime::Service for CrondService {
    async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let mut runner = self.runner;

        let _runner_hdl = tokio::join!(runner.run_jobs());

        Ok(())
    }
}
