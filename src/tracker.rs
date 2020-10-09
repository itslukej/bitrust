use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub type InfoHash = [u8; 20];
pub type PeerId = String;

#[derive(Debug, Clone)]
pub struct Peer {
  ip: SocketAddr,
  uploaded: usize,
  downloaded: usize,
  left: usize
}

pub enum TrackerCommand {
  AddPeer {
    info_hash: InfoHash,
    peer_id: PeerId,
    peer: Peer
  },
  GetPeers {
    info_hash: InfoHash,
    resp: oneshot::Sender<Option<HashMap<PeerId, Peer>>>
  },
}

pub struct Tracker {
  peers: HashMap<InfoHash, HashMap<PeerId, Peer>>,
}

impl Tracker {
  pub fn new() -> TrackerHandle {
    let (tx, rx) = mpsc::unbounded_channel::<TrackerCommand>();

    let tracker = Tracker {
      peers: Default::default()
    };

    tokio::spawn(async move {
      tracker.run_loop(rx).await
    });

    TrackerHandle {
      sender: tx
    }
  }

  async fn run_loop(mut self, mut rx: mpsc::UnboundedReceiver<TrackerCommand>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        TrackerCommand::AddPeer { info_hash, peer, peer_id } => {
          self.peers.entry(info_hash).or_insert(Default::default()).insert(peer_id, peer);
        },
        TrackerCommand::GetPeers { info_hash, resp } => {
          resp.send(self.peers.get(&info_hash).cloned()).ok();
        }
      }
    }
  }
}

#[derive(Clone)]
pub struct TrackerHandle {
  sender: mpsc::UnboundedSender<TrackerCommand>
}

impl TrackerHandle {
  pub async fn add_peer(&self, info_hash: InfoHash, peer_id: PeerId, peer: Peer) -> Result<(), mpsc::error::SendError<TrackerCommand>> {
    self.sender.send(TrackerCommand::AddPeer {
      info_hash,
      peer_id,
      peer
    })
  }

  pub async fn get_peers(&self, info_hash: InfoHash) -> Option<HashMap<PeerId, Peer>> {
    let (tx, rx) = oneshot::channel();

    self.sender.send(TrackerCommand::GetPeers {
      info_hash,
      resp: tx
    }).ok();

    rx.await.unwrap()
  }
}