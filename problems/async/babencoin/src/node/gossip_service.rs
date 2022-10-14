#![forbid(unsafe_code)]

use crate::{
    block_forest::BlockForest,
    data::{BlockHash, TransactionHash, VerifiedBlock, VerifiedPeerMessage, VerifiedTransaction},
    node::mining_service::MiningInfo,
    node::peer_service::{PeerCommand, PeerCommandKind, PeerEvent, PeerEventKind, SessionId},
};

use anyhow::{Context, Result};
use crossbeam::{
    channel::{self, Receiver, Sender},
    select,
};
use log::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Serialize, Deserialize)]
pub struct GossipServiceConfig {
    #[serde(with = "humantime_serde")]
    pub eager_requests_interval: Duration,
}

pub struct GossipService {
    config: GossipServiceConfig,
    event_receiver: Receiver<PeerEvent>,
    command_sender: Sender<PeerCommand>,
    block_receiver: Receiver<VerifiedBlock>,
    mining_info_sender: Sender<MiningInfo>,
    block_forest: BlockForest,
    // TODO: your code goes here.
}

impl GossipService {
    pub fn new(
        config: GossipServiceConfig,
        event_receiver: Receiver<PeerEvent>,
        command_sender: Sender<PeerCommand>,
        block_receiver: Receiver<VerifiedBlock>,
        mining_info_sender: Sender<MiningInfo>,
    ) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn run(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }

    // TODO: your code goes here.
}
