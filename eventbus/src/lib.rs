/**
 * The msg module contains common message definitions for serialization and deserialization of data
 * across the eventbus
 */
extern crate serde;
extern crate serde_json;

use actix::Message;
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::sync::Arc;

/**
 * Default function for deserialize/serialize of times, always defaults to 1970-01-01
 */
fn epoch() -> DateTime<Utc> {
    use chrono::offset::TimeZone;
    return Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444);
}

/**
 * The `Meta` struct contains the necessary metadata about a message which is being sent over the
 * wire
 *
 * It is not intended to carry message contents itself, but rather information about the message
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct Meta {
    pub channel: String,
    #[serde(default = "epoch")]
    pub ts: DateTime<Utc>,
}

/**
 * The Output enums are all meant to capture the types of messages which can be received from the
 * eventbus.
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "type", rename_all = "camelCase")]
#[rtype(result = "()")]
pub enum Output {
    Heartbeat,
    Message { payload: Value },
}

/**
 * OutputMessage is the fully realized and serializable form of an Output
 *
 * This struct should never be constructed except by the websocket handlers just prior to writing
 * to an active websocket
 *
 * Clients should be prepared to handle each of these messages coming over the channels they
 * subscribe to.
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct OutputMessage {
    pub msg: Arc<Output>,
    pub meta: Meta,
}

/**
 * The Input enums are all meant to capture the types of messages that can be send to the eventbus
 * as "inputs."
 *
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "type", rename_all = "camelCase")]
#[rtype(result = "()")]
pub enum Input {
    /**
     * A Subscribe message must be sent for each channel the client wishes to subscribe to.
     *
     * These subscriptions are currently NOT durable. Once the client disconnects, subscriptions
     * will be cleared automatically
     */
    Subscribe {
        /**
         * The client's UUID
         */
        client: String,
    },
    /**
     * The unsubscribe message can be sent if the client wishes to stop following a specific
     * channel, but remained connected and following others.
     */
    Unsubscribe {
        /**
         * The client's UUID
         */
        client: String,
    },
    /**
     * The Publish message is the most common message clients should be sending
     *
     * The `payload` is an arbitrary bit of JSON, and is not typed
     */
    Publish { payload: Value },
}

/**
 * InputMessage is the fully realized and serializable form of an Inputt
 *
 * This struct should never be constructed except by client websocket handlers just prior to
 * writing to an active websocket
 */
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct InputMessage {
    pub msg: Input,
    pub meta: Meta,
}
