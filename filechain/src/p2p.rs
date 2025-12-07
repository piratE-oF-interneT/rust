use crate::models::Block;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, IdentTopic as Topic},
    identity,
    swarm::{Swarm, SwarmEvent},
};

pub async fn start_p2p() {
    let key = identity::Keypair::generate_ed25519();
    let topic = Topic::new("blocks");
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(key)
        .with_tokio()
        .with_tcp()
        .with_dns()
        .with_gossipsub()
        .build()
        .unwrap();

    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    loop {
        match swarm.next().await.unwrap() {
            SwarmEvent::Behaviour(GossipsubEvent::Message { message, .. }) => {
                let block: Block = serde_json::from_slice(&message.data).unwrap();
                println!("Received block {}", block.index);
            }
            _ => {}
        }
    }
}
