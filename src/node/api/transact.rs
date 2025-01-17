use super::messages::{TransactRequest, TransactResponse};
use super::{promote_block, NodeContext, NodeError};
use crate::blockchain::Blockchain;
use crate::core::ChainSourcedTx;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn transact<B: Blockchain>(
    client: Option<SocketAddr>,
    context: Arc<RwLock<NodeContext<B>>>,
    req: TransactRequest,
) -> Result<TransactResponse, NodeError> {
    let mut ctx = context.write().await;
    let is_local = client.map(|c| c.ip().is_loopback()).unwrap_or(false);
    ctx.mempool_add_chain_sourced(is_local, ChainSourcedTx::TransactionAndDelta(req.tx_delta))?;
    if is_local {
        let wallet = ctx.wallet.clone();
        // Invoke PoS block generation
        if let Some(draft) = ctx.try_produce(wallet)? {
            drop(ctx);
            promote_block(context, draft).await;
        }
    }
    Ok(TransactResponse {})
}
