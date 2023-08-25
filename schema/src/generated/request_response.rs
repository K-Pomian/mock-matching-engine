#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestResponse {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub order_id: u64,
    #[prost(uint64, tag = "3")]
    pub user_id: u64,
    #[prost(message, repeated, tag = "4")]
    pub fills: ::prost::alloc::vec::Vec<Fill>,
    #[prost(message, optional, tag = "5")]
    pub maker_fee_amount: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "6")]
    pub taker_fee_amount: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(uint64, tag = "7")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fill {
    #[prost(uint64, tag = "1")]
    pub filled_order_id: u64,
    #[prost(uint64, tag = "2")]
    pub filled_amount: u64,
    #[prost(bool, tag = "3")]
    pub filled_in_full: bool,
}
