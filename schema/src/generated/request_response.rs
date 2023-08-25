#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestResponse {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "2")]
    pub status: i32,
    #[prost(enumeration = "FailureReason", optional, tag = "3")]
    pub failure_reason: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub output: ::core::option::Option<RequestOutput>,
    #[prost(uint64, tag = "5")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOutput {
    #[prost(uint64, tag = "1")]
    pub order_id: u64,
    #[prost(uint64, tag = "2")]
    pub user_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub fills: ::prost::alloc::vec::Vec<Fill>,
    #[prost(message, optional, tag = "4")]
    pub maker_fee_amount: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
    #[prost(message, optional, tag = "5")]
    pub taker_fee_amount: ::core::option::Option<super::proto_decimal::ProtoDecimal>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    Failure = 1,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Success => "SUCCESS",
            Status::Failure => "FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FailureReason {
    NotEnoughLiquidity = 0,
}
impl FailureReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FailureReason::NotEnoughLiquidity => "NotEnoughLiquidity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotEnoughLiquidity" => Some(Self::NotEnoughLiquidity),
            _ => None,
        }
    }
}
