//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use parachain_runtime::{AccountId, AssetBalance, AssetId, Balance, BlockNumber, Moment, Nonce};
use parachains_common::opaque::Block;

use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P, Backend> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// backend instance used to access offchain storage (added by encointer).
	pub backend: Arc<Backend>,
	/// whether offchain-indexing is enabled (added by encointer).
	pub offchain_indexing_enabled: bool,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, TBackend>(
	deps: FullDeps<C, P, TBackend>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + Sync + Send + 'static,
	C::Api: pallet_encointer_ceremonies_rpc_runtime_api::CeremoniesApi<Block, AccountId, Moment>,
	C::Api:
		pallet_encointer_communities_rpc_runtime_api::CommunitiesApi<Block, AccountId, BlockNumber>,
	C::Api: pallet_encointer_bazaar_rpc_runtime_api::BazaarApi<Block, AccountId>,
	C::Api: pallet_encointer_treasuries_rpc_runtime_api::TreasuriesApi<Block, AccountId>,
	C::Api: encointer_balances_tx_payment_rpc_runtime_api::BalancesTxPaymentApi<
		Block,
		Balance,
		AssetId,
		AssetBalance,
	>,
	TBackend: sc_client_api::Backend<Block>, // added by encointer
	<TBackend as sc_client_api::Backend<Block>>::OffchainStorage: 'static, // added by encointer
{
	use encointer_balances_tx_payment_rpc::{BalancesTxPaymentApiServer, BalancesTxPaymentRpc};
	use pallet_encointer_bazaar_rpc::{BazaarApiServer, BazaarRpc};
	use pallet_encointer_ceremonies_rpc::{CeremoniesApiServer, CeremoniesRpc};
	use pallet_encointer_communities_rpc::{CommunitiesApiServer, CommunitiesRpc};
	use pallet_encointer_treasuries_rpc::{TreasuriesApiServer, TreasuriesRpc};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcExtension::new(());
	let FullDeps { client, pool, backend, offchain_indexing_enabled } = deps;

	module.merge(System::new(client.clone(), pool).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(BalancesTxPaymentRpc::new(client.clone()).into_rpc())?;
	module.merge(BazaarRpc::new(client.clone()).into_rpc())?;
	module.merge(TreasuriesRpc::new(client.clone()).into_rpc())?;

	match backend.offchain_storage() {
		Some(storage) => {
			module.merge(
				CommunitiesRpc::new(client.clone(), storage.clone(), offchain_indexing_enabled)
					.into_rpc(),
			)?;

			module
				.merge(CeremoniesRpc::new(client, storage, offchain_indexing_enabled).into_rpc())?;
		},
		None => log::warn!(
			"Offchain caching disabled, due to lack of offchain storage support in backend. \n
			Will not initialize custom RPCs for 'CommunitiesApi' and 'CeremoniesApi'"
		),
	};

	Ok(module)
}
