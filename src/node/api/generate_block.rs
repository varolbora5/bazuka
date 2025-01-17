use super::messages::{GenerateBlockRequest, GenerateBlockResponse};
use super::{promote_block, NodeContext, NodeError};
use crate::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn generate_block<B: Blockchain>(
    context: Arc<RwLock<NodeContext<B>>>,
    _req: GenerateBlockRequest,
) -> Result<GenerateBlockResponse, NodeError> {
    let mut ctx = context.write().await;
    let wallet = ctx.wallet.clone();
    // Invoke PoS block generation
    if let Some(draft) = ctx.try_produce(wallet)? {
        drop(ctx);
        promote_block(context, draft).await;
        return Ok(GenerateBlockResponse { success: true });
    }
    Ok(GenerateBlockResponse { success: false })
}
