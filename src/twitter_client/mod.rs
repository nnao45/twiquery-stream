include!("twitter_client.rs");
include!("exec.rs");
include!("test_server.rs");

extern crate tweetust;
extern crate twitter_stream;
extern crate twitter_stream_message;
extern crate chrono;
extern crate snailquote;
extern crate curl;
extern crate slog_scope;
extern crate tokio_timer;
extern crate serde;
extern crate serde_json as json;
use twitter_stream::{TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream_message::StreamMessage;

use curl::easy::{Easy, List};
use curl::Error as CurlError;

use chrono::Local;

use std::time::Duration;
use std::io::Read;

use snailquote::unescape;

use serde::Deserialize;
use serde::Serialize;

use slog::{slog_info,slog_error};
use slog_scope::{info,error};

