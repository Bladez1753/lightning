
// This file was automatically derived from the JSON-RPC schemas in
// `doc/schemas`. Do not edit this file manually as it would get
// overwritten.

use std::convert::From;
#[allow(unused_imports)]
use cln_rpc::model::{responses,requests};
use crate::pb;

#[allow(unused_variables)]
impl From<&responses::GetinfoAddress> for pb::GetinfoAddress {
    fn from(c: &responses::GetinfoAddress) -> Self {
        Self {
            item_type: c.item_type as i32,
            port: c.port.into(),
            address: c.address.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::GetinfoBinding> for pb::GetinfoBinding {
    fn from(c: &responses::GetinfoBinding) -> Self {
        Self {
            item_type: c.item_type as i32,
            address: c.address.clone(),
            port: c.port.map(|v| v.into()),
            socket: c.socket.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::GetinfoResponse> for pb::GetinfoResponse {
    fn from(c: &responses::GetinfoResponse) -> Self {
        Self {
            id: hex::decode(&c.id).unwrap(),
            alias: c.alias.clone(),
            color: hex::decode(&c.color).unwrap(),
            num_peers: c.num_peers.clone(),
            num_pending_channels: c.num_pending_channels.clone(),
            num_active_channels: c.num_active_channels.clone(),
            num_inactive_channels: c.num_inactive_channels.clone(),
            version: c.version.clone(),
            lightning_dir: c.lightning_dir.clone(),
            blockheight: c.blockheight.clone(),
            network: c.network.clone(),
            fees_collected_msat: Some(c.fees_collected_msat.into()),
            address: c.address.iter().map(|s| s.into()).collect(),
            binding: c.binding.iter().map(|s| s.into()).collect(),
            warning_bitcoind_sync: c.warning_bitcoind_sync.clone(),
            warning_lightningd_sync: c.warning_lightningd_sync.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersPeersLog> for pb::ListpeersPeersLog {
    fn from(c: &responses::ListpeersPeersLog) -> Self {
        Self {
            item_type: c.item_type as i32,
            num_skipped: c.num_skipped.clone(),
            time: c.time.clone(),
            source: c.source.clone(),
            log: c.log.clone(),
            node_id: c.node_id.as_ref().map(|v| hex::decode(&v).unwrap()),
            data: c.data.as_ref().map(|v| hex::decode(&v).unwrap()),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersPeersChannelsInflight> for pb::ListpeersPeersChannelsInflight {
    fn from(c: &responses::ListpeersPeersChannelsInflight) -> Self {
        Self {
            funding_txid: hex::decode(&c.funding_txid).unwrap(),
            funding_outnum: c.funding_outnum.clone(),
            feerate: c.feerate.clone(),
            total_funding_msat: Some(c.total_funding_msat.into()),
            our_funding_msat: Some(c.our_funding_msat.into()),
            scratch_txid: hex::decode(&c.scratch_txid).unwrap(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersPeersChannelsHtlcs> for pb::ListpeersPeersChannelsHtlcs {
    fn from(c: &responses::ListpeersPeersChannelsHtlcs) -> Self {
        Self {
            direction: c.direction as i32,
            id: c.id.clone(),
            amount_msat: Some(c.amount_msat.into()),
            expiry: c.expiry.clone(),
            payment_hash: hex::decode(&c.payment_hash).unwrap(),
            local_trimmed: c.local_trimmed.clone(),
            status: c.status.clone(),
            state: c.state as i32,
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersPeersChannels> for pb::ListpeersPeersChannels {
    fn from(c: &responses::ListpeersPeersChannels) -> Self {
        Self {
            state: c.state as i32,
            scratch_txid: c.scratch_txid.as_ref().map(|v| hex::decode(&v).unwrap()),
            owner: c.owner.clone(),
            short_channel_id: c.short_channel_id.clone(),
            channel_id: c.channel_id.as_ref().map(|v| hex::decode(&v).unwrap()),
            funding_txid: c.funding_txid.as_ref().map(|v| hex::decode(&v).unwrap()),
            initial_feerate: c.initial_feerate.clone(),
            last_feerate: c.last_feerate.clone(),
            next_feerate: c.next_feerate.clone(),
            next_fee_step: c.next_fee_step.clone(),
            inflight: c.inflight.iter().map(|s| s.into()).collect(),
            close_to: c.close_to.as_ref().map(|v| hex::decode(&v).unwrap()),
            private: c.private.clone(),
            opener: c.opener as i32,
            closer: c.closer.map(|v| v as i32),
            features: c.features.iter().map(|s| s.into()).collect(),
            to_us_msat: c.to_us_msat.map(|f| f.into()),
            min_to_us_msat: c.min_to_us_msat.map(|f| f.into()),
            max_to_us_msat: c.max_to_us_msat.map(|f| f.into()),
            total_msat: c.total_msat.map(|f| f.into()),
            fee_base_msat: c.fee_base_msat.map(|f| f.into()),
            fee_proportional_millionths: c.fee_proportional_millionths.clone(),
            dust_limit_msat: c.dust_limit_msat.map(|f| f.into()),
            max_total_htlc_in_msat: c.max_total_htlc_in_msat.map(|f| f.into()),
            their_reserve_msat: c.their_reserve_msat.map(|f| f.into()),
            our_reserve_msat: c.our_reserve_msat.map(|f| f.into()),
            spendable_msat: c.spendable_msat.map(|f| f.into()),
            receivable_msat: c.receivable_msat.map(|f| f.into()),
            minimum_htlc_in_msat: c.minimum_htlc_in_msat.map(|f| f.into()),
            their_to_self_delay: c.their_to_self_delay.clone(),
            our_to_self_delay: c.our_to_self_delay.clone(),
            max_accepted_htlcs: c.max_accepted_htlcs.clone(),
            status: c.status.iter().map(|s| s.into()).collect(),
            in_payments_offered: c.in_payments_offered.clone(),
            in_offered_msat: c.in_offered_msat.map(|f| f.into()),
            in_payments_fulfilled: c.in_payments_fulfilled.clone(),
            in_fulfilled_msat: c.in_fulfilled_msat.map(|f| f.into()),
            out_payments_offered: c.out_payments_offered.clone(),
            out_offered_msat: c.out_offered_msat.map(|f| f.into()),
            out_payments_fulfilled: c.out_payments_fulfilled.clone(),
            out_fulfilled_msat: c.out_fulfilled_msat.map(|f| f.into()),
            htlcs: c.htlcs.iter().map(|s| s.into()).collect(),
            close_to_addr: c.close_to_addr.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersPeers> for pb::ListpeersPeers {
    fn from(c: &responses::ListpeersPeers) -> Self {
        Self {
            id: hex::decode(&c.id).unwrap(),
            connected: c.connected.clone(),
            log: c.log.iter().map(|s| s.into()).collect(),
            channels: c.channels.iter().map(|s| s.into()).collect(),
            netaddr: c.netaddr.iter().map(|s| s.into()).collect(),
            features: c.features.as_ref().map(|v| hex::decode(&v).unwrap()),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListpeersResponse> for pb::ListpeersResponse {
    fn from(c: &responses::ListpeersResponse) -> Self {
        Self {
            peers: c.peers.iter().map(|s| s.into()).collect(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListfundsOutputs> for pb::ListfundsOutputs {
    fn from(c: &responses::ListfundsOutputs) -> Self {
        Self {
            txid: hex::decode(&c.txid).unwrap(),
            output: c.output.clone(),
            amount_msat: Some(c.amount_msat.into()),
            scriptpubkey: hex::decode(&c.scriptpubkey).unwrap(),
            address: c.address.clone(),
            redeemscript: c.redeemscript.as_ref().map(|v| hex::decode(&v).unwrap()),
            status: c.status as i32,
            blockheight: c.blockheight.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListfundsChannels> for pb::ListfundsChannels {
    fn from(c: &responses::ListfundsChannels) -> Self {
        Self {
            peer_id: hex::decode(&c.peer_id).unwrap(),
            our_amount_msat: Some(c.our_amount_msat.into()),
            amount_msat: Some(c.amount_msat.into()),
            funding_txid: hex::decode(&c.funding_txid).unwrap(),
            funding_output: c.funding_output.clone(),
            connected: c.connected.clone(),
            state: c.state as i32,
            short_channel_id: c.short_channel_id.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListfundsResponse> for pb::ListfundsResponse {
    fn from(c: &responses::ListfundsResponse) -> Self {
        Self {
            outputs: c.outputs.iter().map(|s| s.into()).collect(),
            channels: c.channels.iter().map(|s| s.into()).collect(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListchannelsChannels> for pb::ListchannelsChannels {
    fn from(c: &responses::ListchannelsChannels) -> Self {
        Self {
            source: hex::decode(&c.source).unwrap(),
            destination: hex::decode(&c.destination).unwrap(),
            public: c.public.clone(),
            amount_msat: Some(c.amount_msat.into()),
            message_flags: c.message_flags.into(),
            channel_flags: c.channel_flags.into(),
            active: c.active.clone(),
            last_update: c.last_update.clone(),
            base_fee_millisatoshi: c.base_fee_millisatoshi.clone(),
            fee_per_millionth: c.fee_per_millionth.clone(),
            delay: c.delay.clone(),
            htlc_minimum_msat: Some(c.htlc_minimum_msat.into()),
            htlc_maximum_msat: c.htlc_maximum_msat.map(|f| f.into()),
            features: hex::decode(&c.features).unwrap(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::ListchannelsResponse> for pb::ListchannelsResponse {
    fn from(c: &responses::ListchannelsResponse) -> Self {
        Self {
            channels: c.channels.iter().map(|s| s.into()).collect(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::AddgossipResponse> for pb::AddgossipResponse {
    fn from(c: &responses::AddgossipResponse) -> Self {
        Self {
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::AutocleaninvoiceResponse> for pb::AutocleaninvoiceResponse {
    fn from(c: &responses::AutocleaninvoiceResponse) -> Self {
        Self {
            enabled: c.enabled.clone(),
            expired_by: c.expired_by.clone(),
            cycle_seconds: c.cycle_seconds.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::CheckmessageResponse> for pb::CheckmessageResponse {
    fn from(c: &responses::CheckmessageResponse) -> Self {
        Self {
            verified: c.verified.clone(),
            pubkey: c.pubkey.as_ref().map(|v| hex::decode(&v).unwrap()),
        }
    }
}

#[allow(unused_variables)]
impl From<&responses::CloseResponse> for pb::CloseResponse {
    fn from(c: &responses::CloseResponse) -> Self {
        Self {
            item_type: c.item_type as i32,
            tx: c.tx.as_ref().map(|v| hex::decode(&v).unwrap()),
            txid: c.txid.as_ref().map(|v| hex::decode(&v).unwrap()),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::GetinfoRequest> for requests::GetinfoRequest {
    fn from(c: &pb::GetinfoRequest) -> Self {
        Self {
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::ListpeersRequest> for requests::ListpeersRequest {
    fn from(c: &pb::ListpeersRequest) -> Self {
        Self {
            id: c.id.clone().map(|v| hex::encode(v)),
            level: c.level.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::ListfundsRequest> for requests::ListfundsRequest {
    fn from(c: &pb::ListfundsRequest) -> Self {
        Self {
            spent: c.spent.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::ListchannelsRequest> for requests::ListchannelsRequest {
    fn from(c: &pb::ListchannelsRequest) -> Self {
        Self {
            short_channel_id: c.short_channel_id.clone(),
            source: c.source.clone().map(|v| hex::encode(v)),
            destination: c.destination.clone().map(|v| hex::encode(v)),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::AddgossipRequest> for requests::AddgossipRequest {
    fn from(c: &pb::AddgossipRequest) -> Self {
        Self {
            message: hex::encode(&c.message),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::AutocleaninvoiceRequest> for requests::AutocleaninvoiceRequest {
    fn from(c: &pb::AutocleaninvoiceRequest) -> Self {
        Self {
            expired_by: c.expired_by.clone(),
            cycle_seconds: c.cycle_seconds.clone(),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::CheckmessageRequest> for requests::CheckmessageRequest {
    fn from(c: &pb::CheckmessageRequest) -> Self {
        Self {
            message: c.message.clone(),
            zbase: c.zbase.clone(),
            pubkey: c.pubkey.clone().map(|v| hex::encode(v)),
        }
    }
}

#[allow(unused_variables)]
impl From<&pb::CloseRequest> for requests::CloseRequest {
    fn from(c: &pb::CloseRequest) -> Self {
        Self {
            id: hex::encode(&c.id),
            unilateraltimeout: c.unilateraltimeout.clone(),
            destination: c.destination.clone(),
            fee_negotiation_step: c.fee_negotiation_step.clone(),
            wrong_funding: c.wrong_funding.clone().map(|v| hex::encode(v)),
            force_lease_closed: c.force_lease_closed.clone(),
        }
    }
}

