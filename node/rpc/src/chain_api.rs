// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::RpcState;
use blocks::{
    header::json::BlockHeaderJson, tipset_json::TipsetJson, BlockHeader, Tipset, TipsetKeys,
};
use blockstore::BlockStore;
use cid::{json::CidJson, Cid};
use clock::ChainEpoch;
use crypto::DomainSeparationTag;

use jsonrpc_v2::{Data, Error as JsonRpcError, Params};
use message::{
    signed_message,
    unsigned_message::{self, json::UnsignedMessageJson},
    SignedMessage, UnsignedMessage,
};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use wallet::KeyStore;

#[derive(Serialize, Deserialize)]
pub struct BlockMessages {
    #[serde(rename = "BlsMessages", with = "unsigned_message::json::vec")]
    pub bls_msg: Vec<UnsignedMessage>,
    #[serde(rename = "SecpkMessages", with = "signed_message::json::vec")]
    pub secp_msg: Vec<SignedMessage>,
    #[serde(rename = "Cids", with = "cid::json::vec")]
    pub cids: Vec<Cid>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Message {
    #[serde(with = "cid::json")]
    cid: Cid,
    #[serde(with = "unsigned_message::json")]
    message: UnsignedMessage,
}

pub(crate) async fn chain_get_message<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(CidJson,)>,
) -> Result<UnsignedMessageJson, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (CidJson(msg_cid),) = params;
    let ret: UnsignedMessage = data
        .state_manager
        .blockstore()
        .get(&msg_cid)?
        .ok_or("can't find message with that cid")?;
    Ok(UnsignedMessageJson(ret))
}

pub(crate) async fn chain_notify<'a, DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<usize>,
) -> Result<usize, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let data_subscribe = data.chain_store.subscribe().await;
    let index = chain::sub_head_changes(
        data_subscribe,
        &data.chain_store.heaviest_tipset().await,
        params,
        data.events_pubsub.clone(),
    )
    .await?;
    Ok(index)
}

pub(crate) async fn chain_read_obj<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(CidJson,)>,
) -> Result<Vec<u8>, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (CidJson(obj_cid),) = params;
    let ret = data
        .state_manager
        .blockstore()
        .get_bytes(&obj_cid)?
        .ok_or("can't find object with that cid")?;
    Ok(ret)
}

pub(crate) async fn chain_has_obj<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(CidJson,)>,
) -> Result<bool, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (CidJson(obj_cid),) = params;
    Ok(data
        .state_manager
        .blockstore()
        .get_bytes(&obj_cid)?
        .is_some())
}

pub(crate) async fn chain_block_messages<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(CidJson,)>,
) -> Result<BlockMessages, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (CidJson(blk_cid),) = params;
    let blk: BlockHeader = data
        .state_manager
        .blockstore()
        .get(&blk_cid)?
        .ok_or("can't find block with that cid")?;
    let blk_msgs = blk.messages();
    let (unsigned_cids, signed_cids) =
        chain::read_msg_cids(data.state_manager.blockstore(), &blk_msgs)?;
    let (bls_msg, secp_msg) = chain::block_messages_from_cids(
        data.state_manager.blockstore(),
        &unsigned_cids,
        &signed_cids,
    )?;
    let cids = unsigned_cids
        .into_iter()
        .chain(signed_cids)
        .collect::<Vec<_>>();

    let ret = BlockMessages {
        bls_msg,
        secp_msg,
        cids,
    };
    Ok(ret)
}

pub(crate) async fn chain_get_tipset_by_height<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(ChainEpoch, TipsetKeys)>,
) -> Result<TipsetJson, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (height, tsk) = params;
    let ts = chain::tipset_from_keys(data.state_manager.blockstore(), &tsk)?;
    let tss =
        chain::tipset_by_height(data.state_manager.blockstore(), height, &ts, true)?.unwrap_or(ts);
    Ok(TipsetJson(tss))
}

pub(crate) async fn chain_get_genesis<DB, KS>(
    data: Data<RpcState<DB, KS>>,
) -> Result<Option<TipsetJson>, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let genesis =
        chain::genesis(data.state_manager.blockstore())?.ok_or("can't find genesis tipset")?;
    let gen_ts = Tipset::new(vec![genesis])?;
    Ok(Some(TipsetJson(gen_ts)))
}

pub(crate) async fn chain_head<DB, KS>(
    data: Data<RpcState<DB, KS>>,
) -> Result<TipsetJson, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let heaviest = chain::get_heaviest_tipset(data.state_manager.blockstore())?
        .ok_or("can't find heaviest tipset")?;
    Ok(TipsetJson(heaviest))
}

pub(crate) async fn chain_tipset_weight<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(TipsetKeys,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (tsk,) = params;
    let ts = chain::tipset_from_keys(data.state_manager.blockstore(), &tsk)?;
    Ok(ts.weight().to_str_radix(10))
}

pub(crate) async fn chain_get_block<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(CidJson,)>,
) -> Result<BlockHeaderJson, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (CidJson(blk_cid),) = params;
    let blk: BlockHeader = data
        .state_manager
        .blockstore()
        .get(&blk_cid)?
        .ok_or("can't find BlockHeader with that cid")?;
    Ok(BlockHeaderJson(blk))
}

pub(crate) async fn chain_get_tipset<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(TipsetKeys,)>,
) -> Result<TipsetJson, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (tsk,) = params;
    let ts = chain::tipset_from_keys(data.state_manager.blockstore(), &tsk)?;
    Ok(TipsetJson(ts))
}

pub(crate) async fn chain_get_randomness<DB, KS>(
    data: Data<RpcState<DB, KS>>,
    Params(params): Params<(TipsetKeys, i64, ChainEpoch, Vec<u8>)>,
) -> Result<[u8; 32], JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    let (tsk, pers, epoch, entropy) = params;
    Ok(chain::get_chain_randomness(
        data.state_manager.blockstore(),
        &tsk,
        DomainSeparationTag::from_i64(pers).ok_or("invalid DomainSeparationTag")?,
        epoch,
        &entropy,
    )?)
}
