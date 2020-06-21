/**
 * This is the simplest implementation of an Otto Eventbus, which keeps everything
 * only in memory
 */
#[deny(unsafe_code)]
use dashmap::DashMap;
use futures::future::FutureExt;
use log::*;
use meows;
use otto_eventbus::server::*;
use smol;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

async fn run_server(addr: String) -> Result<(), std::io::Error> {
    let eventbus = MemoryBus::default();
    let server = meows::Server::with_state(eventbus);

    info!("Starting eventbus on {}", addr);
    server.serve(addr).await
}

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8105".to_string();
    smol::run(run_server(addr))
}

/**
 * The MemoryBus is a simple implementation of the Eventbus trait for a totally
 * in-memory eventbus. There is no backing store and all data will be lost between
 * restarts of the application.
 *
 * This is the most simple and primitive implementation of the Eventbus trait
 */
struct MemoryBus {
    topics: Arc<DashMap<Topic, Vec<Message>>>,
    offsets: Arc<DashMap<(Topic, CallerId), Offset>>,
}

impl Eventbus for MemoryBus {
    fn pending(
        self: Arc<Self>,
        topic: Topic,
        caller: CallerId,
    ) -> Pin<Box<dyn Future<Output = i64> + Send>> {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            if let Some(msgs) = topics.get(&topic) {
                let latest = msgs.len() as i64;

                if let Some(current) = offsets.get(&(topic, caller)) {
                    return latest - current.value();
                }
                /* If the caller never showed up then our pending is basically everything
                 * in the topic
                 */
                return latest;
            }
            /*
             * If the topic doesn't exist yet, we'll consider there to be
             * zero pending messages
             */
            0
        }
        .boxed()
    }

    fn latest(self: Arc<Self>, topic: Topic) -> Pin<Box<dyn Future<Output = Offset> + Send>> {
        let topics = self.topics.clone();
        async move {
            if let Some(msgs) = topics.get(&topic) {
                return (msgs.len() - 1) as Offset;
            }
            -1
        }
        .boxed()
    }

    fn at(self: Arc<Self>, topic: Topic, offset: Offset, caller: CallerId) -> AsyncOptionMessage {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            if let Some(msgs) = topics.get(&topic) {
                let offset_handle = (topic, caller);

                if msgs.len() > (offset as usize) {
                    let result = msgs[offset as usize].clone();
                    offsets.insert(offset_handle, (offset as Offset) + 1);
                    return Some(result);
                }
            }
            None
        }
        .boxed()
    }

    fn retrieve(self: Arc<Self>, topic: Topic, caller: CallerId) -> AsyncOptionMessage {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            /*
             * If the topic doesn't exist, None!
             */
            if !topics.contains_key(&topic) {
                return None;
            }

            let msgs = topics.get(&topic).unwrap();
            let offset_handle = (topic, caller);

            if !offsets.contains_key(&offset_handle) {
                offsets.insert(offset_handle, 1);
                return Some(msgs[0].clone());
            }

            let offset = offsets.get(&offset_handle).unwrap();
            let off = *offset.value();
            // Need to explicitly drop offset to drop the reference into dashmap
            // and avoid a deadlock in dashmap by our async function call below
            std::mem::drop(offset);
            return self.at(offset_handle.0, off, offset_handle.1).await;
        }
        .boxed()
    }

    fn publish(
        self: Arc<Self>,
        topic: Topic,
        message: Message,
        caller: CallerId,
    ) -> Pin<Box<dyn Future<Output = Result<Offset, ()>> + Send>> {
        let topics = self.topics.clone();

        async move {
            if !topics.contains_key(&topic) {
                topics.insert(topic.clone(), vec![]);
            }

            if let Some(mut msgs) = topics.get_mut(&topic) {
                msgs.push(message);
                /*
                 * TODO: at this point we need to iterate through subscribers and push the message to
                 * them
                 */
                return Ok(msgs.len() as Offset);
            }
            Err(())
        }
        .boxed()
    }

    fn default() -> Arc<Self>
    where
        Self: Sized,
    {
        Arc::new(Self {
            topics: Arc::new(DashMap::default()),
            offsets: Arc::new(DashMap::default()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use smol;

    fn test_topic() -> Topic {
        String::from("test-topic")
    }

    fn test_caller() -> CallerId {
        String::from("test-caller")
    }

    fn test_bus() -> Bus {
        Bus::new(MemoryBus::default())
    }

    #[test]
    fn test_simple_pending() {
        let bus = test_bus();

        let result = smol::run(bus.pending(test_topic(), test_caller()));
        assert_eq!(0, result);
    }

    #[test]
    fn test_publish_pending() {
        let bus = test_bus();
        let msg = String::from("hello world");
        let start_latest = smol::run(bus.latest(test_topic()));

        let pub_res = smol::run(bus.publish(test_topic(), msg, test_caller()));
        assert!(pub_res.is_ok());
        assert!(start_latest < pub_res.unwrap());

        let pending = smol::run(bus.pending(test_topic(), test_caller()));
        assert_eq!(1, pending);
    }

    #[test]
    fn test_retrieve_no_topic() {
        let bus = test_bus();
        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_publish_retrieve() {
        let bus = test_bus();

        let pub_res = smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));
        assert!(pub_res.is_ok());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());
    }

    #[test]
    fn test_publish_retrieve_twice() {
        let bus = test_bus();

        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller())).unwrap();
        smol::run(bus.publish(test_topic(), "world".to_string(), test_caller())).unwrap();

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("world", retrieved.unwrap());
    }

    #[test]
    fn test_multiple_retrieve() {
        let bus = test_bus();

        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller())).unwrap();

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_latest_empty() {
        let bus = test_bus();
        let result = smol::run(bus.latest(test_topic()));
        assert_eq!(-1, result);
    }

    #[test]
    fn test_latest_with_data() {
        let bus = test_bus();
        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller())).unwrap();
        let result = smol::run(bus.latest(test_topic()));
        assert_eq!(0, result);
    }

    #[test]
    fn test_at_empty() {
        let bus = test_bus();
        let result = smol::run(bus.at(test_topic(), 0, test_caller()));
        assert!(result.is_none());
    }

    #[test]
    fn test_at_with_data() {
        let bus = test_bus();
        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller())).unwrap();

        let result = smol::run(bus.at(test_topic(), 0, test_caller()));
        assert!(result.is_some());
    }
}
