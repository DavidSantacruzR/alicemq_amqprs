
//! # AliceMQ
//! `AliceMQ` is a wrapper built on top of the amqprs library. This provides a collection
//! of utilities designed to simplify the creation of a Consumer / Publisher.

pub mod consumer;
pub mod callback;
pub mod publisher;
mod connection_arguments;
mod constants;
