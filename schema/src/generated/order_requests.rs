#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceMarketOrderRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub user_id: u64,
    #[prost(message, optional, tag = "3")]
    pub size: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "4")]
    pub max_price: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(enumeration = "OrderSide", tag = "5")]
    pub order_side: i32,
    #[prost(message, optional, tag = "6")]
    pub maker_fee: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "7")]
    pub taker_fee: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(uint64, tag = "8")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceLimitOrderRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub user_id: u64,
    #[prost(message, optional, tag = "3")]
    pub size: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(enumeration = "OrderSide", tag = "5")]
    pub order_side: i32,
    #[prost(message, optional, tag = "6")]
    pub maker_fee: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "7")]
    pub taker_fee: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(uint64, tag = "8")]
    pub timestamp: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    Ask = 0,
    Bid = 1,
}
impl OrderSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderSide::Ask => "ASK",
            OrderSide::Bid => "BID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASK" => Some(Self::Ask),
            "BID" => Some(Self::Bid),
            _ => None,
        }
    }
}
