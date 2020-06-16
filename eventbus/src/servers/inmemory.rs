/**
 * This is the simplest implementation of an Otto Eventbus, which keeps everything
 * only in memory
 */
use futures::future::FutureExt;
use log::*;
use otto_eventbus::server::*;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), std::io::Error> {
    Ok(())
}

/**
 * The MemoryBus is a simple implementation of the Eventbus trait for a totally
 * in-memory eventbus. There is no backing store and all data will be lost between
 * restarts of the application.
 *
 * This is the most simple and primitive implementation of the Eventbus trait
 */
struct MemoryBus {
    topics: Arc<RwLock<HashMap<Topic, Vec<Message>>>>,
    offsets: Arc<RwLock<HashMap<(Topic, CallerId), Offset>>>,
}

impl Eventbus for MemoryBus {
    fn pending(&self, topic: Topic, caller: CallerId) -> Pin<Box<dyn Future<Output = i64> + Send>> {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            if let Ok(topics) = topics.read() {
                /*
                 * If the topic doesn't exist yet, we'll consider there to be
                 * zero pending messages
                 */
                if !topics.contains_key(&topic) {
                    return 0;
                }

                let latest = topics[&topic].len() as i64;

                if let Ok(offsets) = offsets.read() {
                    if let Some(current) = offsets.get(&(topic, caller)) {
                        return latest - current;
                    } else {
                        /* If the caller never showed up then our pending is basically everything
                         * in the topic
                         */
                        return latest;
                    }
                } else {
                    error!("Failed to get a read lock on the offsets");
                }
            } else {
                error!("Failed to get a read lock on the topics");
            }
            0
        }
        .boxed()
    }

    fn latest(&self, topic: Topic) -> Pin<Box<dyn Future<Output = Offset> + Send>> {
        let topics = self.topics.clone();
        async move {
            if let Ok(topics) = topics.read() {
                if topics.contains_key(&topic) {
                    return (topics[&topic].len() - 1) as Offset;
                }
            }
            -1
        }
        .boxed()
    }

    fn at(&self, topic: Topic, offset: Offset, caller: CallerId) -> AsyncOptionMessage {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            if let Ok(mut offsets) = offsets.write() {
                if let Ok(topics) = topics.read() {
                    if !topics.contains_key(&topic) {
                        return None;
                    }
                    let offset_handle = (topic, caller);

                    if topics[&offset_handle.0].len() > (offset as usize) {
                        let result = Some(topics[&offset_handle.0][offset as usize].clone());
                        offsets.insert(offset_handle, (offset as Offset) + 1);
                        return result;
                    }
                }
            }
            None
        }
        .boxed()
    }

    fn retrieve(&self, topic: Topic, caller: CallerId) -> AsyncOptionMessage {
        let topics = self.topics.clone();
        let offsets = self.offsets.clone();

        async move {
            if let Ok(mut offsets) = offsets.write() {
                if let Ok(topics) = topics.read() {
                    /*
                     * If the topic doesn't exist, None!
                     */
                    if !topics.contains_key(&topic) {
                        return None;
                    }

                    let offset_handle = (topic, caller);

                    /*
                     * This caller has never read from this topic, so give them the
                     * first message
                     */
                    if !offsets.contains_key(&offset_handle) {
                        let result = Some(topics[&offset_handle.0][0].clone());
                        offsets.insert(offset_handle, 1);
                        return result;
                    } else {
                        /*
                         * This is basically duplicate functionality to what is in .at
                         * unfortunately I am not yet smart enough to figure out
                         * how to invoke other functions on this self from within
                         * the async block just yet
                         *
                         * Whenever I get back to this, the invoke of at() will
                         * have to come out of this block so that the RwLockGuard
                         * is properly dropped before invoking at().await, since
                         * it cannot live across the await
                         */
                        let offset = offsets[&offset_handle] as usize;
                        if topics[&offset_handle.0].len() > offset {
                            let result = Some(topics[&offset_handle.0][offset as usize].clone());
                            offsets.insert(offset_handle, (offset as Offset) + 1);
                            return result;
                        }
                    }
                }
            }
            None
        }
        .boxed()
    }

    fn publish(
        &mut self,
        topic: Topic,
        message: Message,
        caller: CallerId,
    ) -> Pin<Box<dyn Future<Output = Result<Offset, ()>> + Send>> {
        let topics = self.topics.clone();

        async move {
            if let Ok(mut topics) = topics.write() {
                if !topics.contains_key(&topic) {
                    topics.insert(topic.clone(), vec![]);
                }

                if let Some(input) = topics.get_mut(&topic) {
                    input.push(message);
                    return Ok(input.len() as Offset);
                }
            }
            Err(())
        }
        .boxed()
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::default())),
            offsets: Arc::new(RwLock::new(HashMap::default())),
        }
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

    #[test]
    fn test_simple_pending() {
        let bus = MemoryBus::default();

        let result = smol::run(bus.pending(test_topic(), test_caller()));
        assert_eq!(0, result);
    }

    #[test]
    fn test_publish_pending() {
        let mut bus = MemoryBus::default();
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
        let bus = MemoryBus::default();
        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_publish_retrieve() {
        let mut bus = MemoryBus::default();

        let pub_res = smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));
        assert!(pub_res.is_ok());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());
    }

    #[test]
    fn test_publish_retrieve_twice() {
        let mut bus = MemoryBus::default();

        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));
        smol::run(bus.publish(test_topic(), "world".to_string(), test_caller()));

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("world", retrieved.unwrap());
    }

    #[test]
    fn test_multiple_retrieve() {
        let mut bus = MemoryBus::default();

        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = smol::run(bus.retrieve(test_topic(), test_caller()));
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_latest_empty() {
        let bus = MemoryBus::default();
        let result = smol::run(bus.latest(test_topic()));
        assert_eq!(-1, result);
    }

    #[test]
    fn test_latest_with_data() {
        let mut bus = MemoryBus::default();
        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));
        let result = smol::run(bus.latest(test_topic()));
        assert_eq!(0, result);
    }

    #[test]
    fn test_at_empty() {
        let bus = MemoryBus::default();
        let result = smol::run(bus.at(test_topic(), 0, test_caller()));
        assert!(result.is_none());
    }

    #[test]
    fn test_at_with_data() {
        let mut bus = MemoryBus::default();
        smol::run(bus.publish(test_topic(), "hello".to_string(), test_caller()));

        let result = smol::run(bus.at(test_topic(), 0, test_caller()));
        assert!(result.is_some());
    }
}
