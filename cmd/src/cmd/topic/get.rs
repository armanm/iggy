use crate::cli::CliCommand;

use anyhow::{Context, Error, Result};
use async_trait::async_trait;
use comfy_table::Table;
use iggy::client::Client;
use iggy::identifier::Identifier;
use iggy::topics::get_topic::GetTopic;
use iggy::utils::timestamp::TimeStamp;
use tracing::info;

#[derive(Debug)]
pub(crate) struct TopicGet {
    stream_id: u32,
    topic_id: u32,
}

impl TopicGet {
    pub(crate) fn new(stream_id: u32, topic_id: u32) -> Self {
        Self {
            stream_id,
            topic_id,
        }
    }
}

#[async_trait]
impl CliCommand for TopicGet {
    fn explain(&self) -> String {
        format!(
            "get topic with ID: {} from stream with ID: {}",
            self.topic_id, self.stream_id
        )
    }

    async fn execute_cmd(&mut self, client: &dyn Client) -> Result<(), Error> {
        let topic = client
            .get_topic(&GetTopic {
                stream_id: Identifier::numeric(self.stream_id)
                    .expect("Expected numeric identifier for stream_id"),
                topic_id: Identifier::numeric(self.topic_id)
                    .expect("Expected numeric identifier for topic_id"),
            })
            .await
            .with_context(|| {
                format!(
                    "Problem getting topic with ID: {} in stream {}",
                    self.topic_id, self.stream_id
                )
            })?;

        let mut table = Table::new();

        table.set_header(vec!["Property", "Value"]);
        table.add_row(vec!["Topic id", format!("{}", topic.id).as_str()]);
        table.add_row(vec![
            "Created",
            TimeStamp::from(topic.created_at)
                .to_string("%Y-%m-%d %H:%M:%S")
                .as_str(),
        ]);
        table.add_row(vec!["Topic name", topic.name.as_str()]);
        table.add_row(vec!["Topic size", format!("{}", topic.size_bytes).as_str()]);
        table.add_row(vec![
            "Message expiry",
            match topic.message_expiry {
                Some(value) => format!("{}", value),
                None => String::from("None"),
            }
            .as_str(),
        ]);
        table.add_row(vec![
            "Topic message count",
            format!("{}", topic.messages_count).as_str(),
        ]);
        table.add_row(vec![
            "Partitions count",
            format!("{}", topic.partitions_count).as_str(),
        ]);

        info!("{table}");

        info!("{:?}", topic);

        Ok(())
    }
}
