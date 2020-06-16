/**
 * The main eventbus module
 */
use log::*;

// TODO
pub mod client {}

pub mod server {
    use std::future::Future;
    use std::pin::Pin;

    pub type Offset = i64;
    pub type Topic = String;
    pub type CallerId = String;
    pub type Message = String;

    pub type AsyncOptionMessage = Pin<Box<dyn Future<Output = Option<Message>> + Send + 'static>>;

    /**
     * The Eventbus trait should be implemented by the servers which need to
     * implement their own backing stores for an Otto eventbus.
     *
     * Each function is expected to return some sort of future that the caller
     * can await upon.
     *
     * Additionally, many functions will accept the caller's identifier, which
     * can be used in many cases to help differentiate between consumers of the same
     * topic, such as with Kafka consumer groups
     */
    pub trait Eventbus {
        fn pending(
            &self,
            topic: Topic,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = i64> + Send>>;
        /**
         * Fetch the latest offset for the given topic
         */
        fn latest(&self, topic: Topic) -> Pin<Box<dyn Future<Output = Offset> + Send>>;

        /**
         * Retrieve the message at the specified offset
         */
        fn at(&self, topic: Topic, offset: Offset, caller: CallerId) -> AsyncOptionMessage;

        /**
         * Retrieve the latest message
         */
        fn retrieve(&self, topic: Topic, caller: CallerId) -> AsyncOptionMessage;

        /**
         * Publish a message to the given topic
         *
         * Will return a Result, if the publish was success the Ok will contain the
         * new latest offset on the topic
         */
        fn publish(
            &mut self,
            topic: Topic,
            message: Message,
            caller: CallerId,
        ) -> Pin<Box<dyn Future<Output = Result<Offset, ()>> + Send>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
