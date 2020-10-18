/**
 * This module contains the majority of the in-memory eventbus functionality
 */
use dashmap::DashMap;
use futures::channel::mpsc::Sender;
use futures::future::FutureExt;
use futures::sink::SinkExt;
use log::*;
use meows;
use otto_eventbus::message;
use otto_eventbus::server::*;
use parking_lot::Mutex;
use serde_json;
use smol;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

pub type EventbusServer = meows::Server<Arc<MemoryBus>, ()>;

/**
 * Create a valid server for use
 */
pub fn create_server() -> EventbusServer {
    let eventbus = MemoryBus::default();
    let mut server = meows::Server::<Arc<MemoryBus>, ()>::with_state(eventbus);

    server.on("register", register_client);
    server.on("subscribe", subscribe_client);
    server.on("publish", publish_message);
    server.default(default_handler);
    server
}

pub async fn run_server(mut server: EventbusServer, addr: String) -> Result<(), std::io::Error> {
    server.serve(addr).await
}

async fn default_handler(message: String, _state: Arc<Arc<MemoryBus>>) -> Option<meows::Message> {
    warn!("Received a message I cannot handle: {}", message);
    None
}

async fn register_client(mut req: meows::Request<Arc<MemoryBus>, ()>) -> Option<meows::Message> {
    if let Some(register) = req.from_value::<message::Register>() {
        info!("Registration received for client {}", register.uuid);

        /*
         * Check for a duplicate registration
         */
        if req.state.clients.contains_key(&register.uuid) {
            let error = otto_eventbus::message::Error {
                code: "eventbus.already_registered".to_string(),
                data: None,
            };

            return Some(meows::Message::text(
                serde_json::to_string(&error).expect("Failed to serialize error"),
            ));
        }

        /*
         * Just using a random uuid as the authentication token, I don't beleive
         * we need much more "security" in these tokens other than that.
         */
        let token = Uuid::new_v4();
        let response = message::Registered { token };

        let client = Client {
            token,
            sink: req.sink.clone(),
        };

        req.state.add_client(register.uuid, client);

        // TODO: implement a TryFrom or TryInto for the messages defined by the eventbus
        Some(meows::Message::text(
            serde_json::to_string(&response).expect("Failed to parse registered"),
        ))
    } else {
        None
    }
}

async fn subscribe_client(mut req: meows::Request<Arc<MemoryBus>, ()>) -> Option<meows::Message> {
    if let Some(subscribe) = req.from_value::<message::Subscribe>() {
        info!("Subscribe received: {:?}", subscribe);

        // If the client sends an invalid header, bail out early
        if !req
            .state
            .validate_client(&subscribe.header.uuid, &subscribe.header.token)
        {
            warn!("Client could not be validated, perhaps the wrong token?");
            // TODO: make a useful error
            return None;
        }

        info!(
            "Subscribing {} to {}",
            subscribe.header.uuid, subscribe.channel
        );
        req.state
            .subscribe_client(&subscribe.header.uuid, subscribe.channel);

        // TODO: What is the right protocol response for a subscribe?
        //Some(meows::Message::text("ack"))
        None
    } else {
        None
    }
}

async fn publish_message(mut req: meows::Request<Arc<MemoryBus>, ()>) -> Option<meows::Message> {
    if let Some(publish) = req.from_value::<message::Publish>() {
        info!("Publish received: {:?}", publish);
        if !req
            .state
            .validate_client(&publish.header.uuid, &publish.header.token)
        {
            warn!("Client could not be validated, perhaps the wrong token?");
            // TODO: make a useful error
            return None;
        }

        req.state.publish_message(publish.channel, publish.value);
        return Some(meows::Message::text("ack"));
    }
    None
}

/**
 * Utility type to help keep track of sinks where messages are dropped in
 */
type MessageSink = Sender<meows::Message>;

/**
 * The ClientId is the agreed upon uuid that the client(s) will use to identify
 * their registration, subscription, etc requests
 */
type ClientId = Uuid;
/**
 * The client struct represents the server side state the eventbus needs to
 * affiliate with each websocket connection.
 */
pub struct Client {
    /**
     * The token is used to verify protected actions from the client
     */
    token: Uuid,
    /**
     * The sink allows for writing Message objects back to the client's
     * connected websocket
     */
    sink: MessageSink,
}

/**
 * The MemoryBus is a simple implementation of the Engine trait for a totally
 * in-memory eventbus. There is no backing store and all data will be lost between
 * restarts of the application.
 *
 * This is the most simple and primitive implementation of the Engine trait
 */
pub struct MemoryBus {
    /**
     * Clients is a map of identified client structs
     *
     * Each of these clients represents a currently connected client
     */
    clients: Arc<DashMap<ClientId, Client>>,
    /**
     * Channels is a map of topic, to [MessageSink] couplings
     *
     * Channels allow for pub/sub style messaging by the client
     */
    channels: Arc<DashMap<Topic, Vec<MessageSink>>>,
    /**
     * Topics are the commit logs which exist for each named topic
     *
     * This allows for retrieval of messages by specific offsets
     */
    topics: Arc<DashMap<Topic, Vec<Message>>>,
    /**
     * Offsets tracks the caller's offsets for each given topic
     *
     * Offsets do not work or make sense for clients who wish to consume pub-sub
     * style at this point
     */
    offsets: Arc<DashMap<(Topic, CallerId), Offset>>,
}

impl MemoryBus {
    /**
     * Add the client to the internal tracking map, this is assuming that the
     * client's request has already been verified
     */
    pub fn add_client(&self, id: ClientId, client: Client) {
        self.clients.insert(id, client);
    }

    /**
     * Ensure that the provided token matches the client the MemoryBus has in
     * its internal map
     */
    pub fn validate_client(&self, id: &ClientId, token: &Uuid) -> bool {
        if !self.clients.contains_key(id) {
            warn!("Could not validate client {}, they don't exist!", id);
            return false;
        }
        if let Some(client) = self.clients.get(id) {
            return client.token == *token;
        }
        error!("Could not discover the client in order to validate it, this shouldn't happen");
        false
    }

    /**
     * Add the given client's sink to internal channels for receiving messages
     * in a pub/sub manner
     */
    pub fn subscribe_client(&self, id: &ClientId, channel: String) -> Result<(), ()> {
        if let Some(client) = self.clients.get(id) {
            let sink = Mutex::new(client.sink.clone());
            let sink = client.sink.clone();
            if let Some(mut sinks) = self.channels.get_mut(&channel) {
                sinks.push(sink);
            } else {
                let mut sinks = vec![];
                sinks.push(sink);
                self.channels.insert(channel, sinks);
            }
        }
        Ok(())
    }

    pub fn publish_message(&self, channel: String, value: serde_json::Value) {
        info!("publish_message({}, {})", channel, value);

        if let Some(mut sinks) = self.channels.get_mut(&channel) {
            // TODO: make a real error
            let serialized = value.to_string();
            for sink in sinks.iter_mut() {
                let msg = meows::Message::text(serialized.clone());
                /*
                 * TODO: Need to figure out how to wrap this safely in an async
                 * task or wrap self.channels' keys in Arc<Vec<MessageSink>> rather
                 * than just Vec<MessageSink>
                 *
                 * Another stupid idea might be to put the values of the channels
                 * map into their own map Either way, need to safely iterate thorugh the sinks in
                 * order to send all these messages out
                 */
                sink.send(msg).await;
            }
        }
    }
}

impl Engine for MemoryBus {
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
        _caller: CallerId,
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
            channels: Arc::new(DashMap::default()),
            clients: Arc::new(DashMap::default()),
            topics: Arc::new(DashMap::default()),
            offsets: Arc::new(DashMap::default()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::channel::mpsc::channel;
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

    #[async_std::test]
    async fn test_simple_pending() {
        let bus = test_bus();

        let result = bus.pending(test_topic(), test_caller()).await;
        assert_eq!(0, result);
    }

    #[async_std::test]
    async fn test_publish_pending() {
        let bus = test_bus();
        let msg = String::from("hello world");
        let start_latest = bus.latest(test_topic()).await;

        let pub_res = bus.publish(test_topic(), msg, test_caller()).await;
        assert!(pub_res.is_ok());
        assert!(start_latest < pub_res.unwrap());

        let pending = smol::run(bus.pending(test_topic(), test_caller()));
        assert_eq!(1, pending);
    }

    #[async_std::test]
    async fn test_retrieve_no_topic() {
        let bus = test_bus();
        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_none());
    }

    #[async_std::test]
    async fn test_publish_retrieve() {
        let bus = test_bus();

        let pub_res = bus
            .publish(test_topic(), "hello".to_string(), test_caller())
            .await;
        assert!(pub_res.is_ok());

        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());
    }

    #[async_std::test]
    async fn test_publish_retrieve_twice() {
        let bus = test_bus();

        bus.publish(test_topic(), "hello".to_string(), test_caller())
            .await;
        bus.publish(test_topic(), "world".to_string(), test_caller())
            .await;

        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_some());
        assert_eq!("world", retrieved.unwrap());
    }

    #[async_std::test]
    async fn test_multiple_retrieve() {
        let bus = test_bus();

        bus.publish(test_topic(), "hello".to_string(), test_caller())
            .await;

        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_some());
        assert_eq!("hello", retrieved.unwrap());

        let retrieved = bus.retrieve(test_topic(), test_caller()).await;
        assert!(retrieved.is_none());
    }

    #[async_std::test]
    async fn test_latest_empty() {
        let bus = test_bus();
        let result = bus.latest(test_topic()).await;
        assert_eq!(-1, result);
    }

    #[async_std::test]
    async fn test_latest_with_data() {
        let bus = test_bus();
        bus.publish(test_topic(), "hello".to_string(), test_caller())
            .await;
        let result = bus.latest(test_topic()).await;
        assert_eq!(0, result);
    }

    #[async_std::test]
    async fn test_at_empty() {
        let bus = test_bus();
        let result = bus.at(test_topic(), 0, test_caller()).await;
        assert!(result.is_none());
    }

    #[async_std::test]
    async fn test_at_with_data() {
        let bus = test_bus();
        bus.publish(test_topic(), "hello".to_string(), test_caller())
            .await;

        let result = bus.at(test_topic(), 0, test_caller()).await;
        assert!(result.is_some());
    }

    #[test]
    fn test_validate_client() {
        let bus = MemoryBus::default();
        // create a test sink, won't ever be used
        let (sender, _) = channel(1);
        let token = Uuid::new_v4();
        let client = Client {
            token,
            sink: sender,
        };
        let id = Uuid::new_v4();

        bus.add_client(id, client);

        assert!(bus.validate_client(&id, &token));
        assert!(!bus.validate_client(&id, &Uuid::new_v4()));
    }
}
