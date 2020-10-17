/**
 * The main eventbus module
 */

#[macro_use]
extern crate serde_derive;

// TODO
pub mod client {}

pub mod message {
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Register {
        pub uuid: uuid::Uuid,
        pub token: Option<uuid::Uuid>,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Registered {
        pub token: uuid::Uuid,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Error {
        pub code: String,
        pub data: Option<HashMap<String, String>>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Subscribe {
        pub header: ClientHeader,
        pub channel: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Publish {
        pub header: ClientHeader,
        pub channel: String,
        pub value: serde_json::Value,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientHeader {
        pub uuid: uuid::Uuid,
        pub token: uuid::Uuid,
    }
}

pub mod server {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;

    pub type Offset = i64;
    pub type Topic = String;
    pub type CallerId = String;
    pub type Message = String;

    pub type AsyncOptionMessage = Pin<Box<dyn Future<Output = Option<Message>> + Send + 'static>>;

    pub struct Bus {
        inner: Arc<dyn Engine>,
    }

    impl Bus {
        pub fn new(inner: Arc<dyn Engine>) -> Self {
            Self { inner }
        }
        pub fn pending(
            &self,
            topic: Topic,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = i64> + Send>> {
            self.inner.clone().pending(topic, caller)
        }
        pub fn latest(&self, topic: Topic) -> Pin<Box<dyn Future<Output = Offset> + Send>> {
            self.inner.clone().latest(topic)
        }
        pub fn at(&self, topic: Topic, offset: Offset, caller: CallerId) -> AsyncOptionMessage {
            self.inner.clone().at(topic, offset, caller)
        }
        pub fn retrieve(&self, topic: Topic, caller: CallerId) -> AsyncOptionMessage {
            self.inner.clone().retrieve(topic, caller)
        }
        pub fn publish(
            &self,
            topic: Topic,
            message: Message,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = Result<Offset, ()>> + Send>> {
            self.inner.clone().publish(topic, message, caller)
        }
    }

    /**
     * The Engine trait should be implemented by the servers which need to
     * implement their own backing stores for an Otto eventbus.
     *
     * Each function is expected to return some sort of future that the caller
     * can await upon.
     *
     * Additionally, many functions will accept the caller's identifier, which
     * can be used in many cases to help differentiate between consumers of the same
     * topic, such as with Kafka consumer groups
     */
    pub trait Engine {
        /**
         * Return the number of messages the caller has not yet consumed
         */
        fn pending(
            self: Arc<Self>,
            topic: Topic,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = i64> + Send>>;

        /**
         * Fetch the latest offset for the given topic
         *
         * This should increment the caller's offset
         */
        fn latest(self: Arc<Self>, topic: Topic) -> Pin<Box<dyn Future<Output = Offset> + Send>>;

        /**
         * Retrieve the message at the specified offset
         */
        fn at(
            self: Arc<Self>,
            topic: Topic,
            offset: Offset,
            caller: CallerId,
        ) -> AsyncOptionMessage;

        /**
         * Retrieve the latest message
         */
        fn retrieve(self: Arc<Self>, topic: Topic, caller: CallerId) -> AsyncOptionMessage;

        /**
         * Publish a message to the given topic
         *
         * Will return a Result, if the publish was success the Ok will contain the
         * new latest offset on the topic
         */
        fn publish(
            self: Arc<Self>,
            topic: Topic,
            message: Message,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = Result<Offset, ()>> + Send>>;

        /**
         * Return a wrapped version of the implementation which can be invoked
         *
         * This is the preferred means of creating things
         */
        fn default() -> Arc<Self>
        where
            Self: Sized;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
