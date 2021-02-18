//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
// use sp_core::Bytes;
use std::sync::Arc;
use wyvern_exchange_runtime_api::WyvernExchangeApi as WyvernExchangeRuntimeApi;
use wyvern_exchange::{Side,SaleKind,FeeMethod,HowToCall};
use codec::Codec;

#[rpc]
pub trait WyvernExchangeApi<BlockHash,AccountId,Balance,Moment,Signature> {
	#[rpc(name = "wyvernExchange_calculateFinalPriceEx")]
  fn calculate_final_price_ex(&self,
        side: Side,
        sale_kind: SaleKind,
        base_price: u64,
        extra: Moment,
        listing_time: Moment,
        expiration_time: Moment,
    at: Option<BlockHash>) -> Result<u64>;

	#[rpc(name = "wyvernExchange_hashOrderEx")]
    fn hash_order_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<BlockHash>) -> Result<Vec<u8>>;

#[rpc(name = "wyvernExchange_hashToSignEx")]
    fn hash_to_sign_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<BlockHash>) -> Result<Vec<u8>>;

#[rpc(name = "wyvernExchange_validateOrderParametersEx")]
   fn validate_order_parameters_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<BlockHash>) -> Result<bool>;

#[rpc(name = "wyvernExchange_validateOrderEx")]
   fn validate_order_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: Signature,
    at: Option<BlockHash>) -> Result<bool> ;

#[rpc(name = "wyvernExchange_calculateCurrentPriceEx")]
 fn calculate_current_price_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<BlockHash>) -> Result<u64>;

#[rpc(name = "wyvernExchange_ordersCanMatchEx")]
   fn orders_can_match_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
    at: Option<BlockHash>) -> Result<bool> ;

#[rpc(name = "wyvernExchange_calculateMatchPriceEx")]
fn calculate_match_price_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
    at: Option<BlockHash>) -> Result<u64> ;
}

/// A struct that implements the `WyvernExchangeApi`.
pub struct WyvernExchange<C, M> {
	// If you have more generics, no need to WyvernExchange<C, M, N, P, ...>
	// just use a tuple like WyvernExchange<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> WyvernExchange<C, M> {
	/// Create new `WyvernExchange` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block,AccountId,Balance,Moment,Signature> WyvernExchangeApi<<Block as BlockT>::Hash,AccountId,Balance,Moment,Signature> for WyvernExchange<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: WyvernExchangeRuntimeApi<Block,AccountId,Balance,Moment,Signature>,
    AccountId:Codec,
    Balance:Codec,
    Moment:Codec,
    Signature:Codec,
{
 fn calculate_final_price_ex(&self,
        side: Side,
        sale_kind: SaleKind,
        base_price: u64,
        extra: Moment,
        listing_time: Moment,
        expiration_time: Moment,
    at: Option<<Block as BlockT>::Hash>) -> Result<u64>{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.calculate_final_price_ex(&at,  side,
        sale_kind,
        base_price,
        extra,
        listing_time,
        expiration_time);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
    fn hash_order_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<Vec<u8>>{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.hash_order_ex(&at,
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata.clone().into_bytes(),
        replacement_pattern.clone().into_bytes(),
        static_extradata.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

    fn hash_to_sign_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<Vec<u8>>{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.hash_to_sign_ex(&at,
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata.clone().into_bytes(),
        replacement_pattern.clone().into_bytes(),
        static_extradata.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
   fn validate_order_parameters_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<bool>{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.validate_order_parameters_ex(&at,
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata.clone().into_bytes(),
        replacement_pattern.clone().into_bytes(),
        static_extradata.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
   fn validate_order_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: Signature,
    at: Option<<Block as BlockT>::Hash>) -> Result<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.validate_order_ex(&at,
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata.clone().into_bytes(),
        replacement_pattern.clone().into_bytes(),
        static_extradata.clone().into_bytes(),
        sig);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
 fn calculate_current_price_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<u64>{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.calculate_current_price_ex(&at,
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata.clone().into_bytes(),
        replacement_pattern.clone().into_bytes(),
        static_extradata.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
   fn orders_can_match_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.orders_can_match_ex(&at,
        addrs,
        uints,
        fee_methods_sides_kinds_how_to_calls.clone().into_bytes(),
        calldata_buy.clone().into_bytes(),
        calldata_sell.clone().into_bytes(),
        replacement_pattern_buy.clone().into_bytes(),
        replacement_pattern_sell.clone().into_bytes(),
        static_extradata_buy.clone().into_bytes(),
        static_extradata_sell.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
fn calculate_match_price_ex(&self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
    at: Option<<Block as BlockT>::Hash>) -> Result<u64> 
{
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.calculate_match_price_ex(&at,
        addrs,
        uints,
        fee_methods_sides_kinds_how_to_calls.clone().into_bytes(),
        calldata_buy.clone().into_bytes(),
        calldata_sell.clone().into_bytes(),
        replacement_pattern_buy.clone().into_bytes(),
        replacement_pattern_sell.clone().into_bytes(),
        static_extradata_buy.clone().into_bytes(),
        static_extradata_sell.clone().into_bytes());
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
