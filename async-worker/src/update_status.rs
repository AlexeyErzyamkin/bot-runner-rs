use {
    std::{
        time::Duration
    },
    ::tokio::{
        time
    },
    ::surf::{
        self,
        url::Url
    },
    shared::{
        models::{
            WorkerInfo,
            WorkerAction
        }
    },
    crate::{
        Tx,
        WorkerCommand
    },

};

pub async fn update_status(mut tx: Tx, state_url: Url) -> std::io::Result<()> {
    let mut prev_status = WorkerInfo::default();

    loop {
        match surf::get(&state_url).recv_json::<WorkerInfo>().await {
            Ok(status) => {
                if status.version > prev_status.version {
                    tx.send(WorkerCommand::Stop).await;

                    if status.update_version > prev_status.update_version {
                        tx.send(WorkerCommand::Update(status.update_version)).await;
                    }

                    match status.action {
                        WorkerAction::Start(ref start_info) => {
                            tx.send(WorkerCommand::Start(start_info.clone())).await;
                        },
                        WorkerAction::Stop => (),
                        WorkerAction::Update => ()
                    };
                }

                prev_status = status;
            }
            Err(e) => {
                eprintln!("Error getting status update: {:?}", e);

                time::delay_for(Duration::from_secs(30)).await;
            }
        }

        time::delay_for(Duration::from_secs(10)).await;
    }
}