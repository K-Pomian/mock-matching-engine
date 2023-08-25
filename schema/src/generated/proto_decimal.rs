#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDecimal {
    #[prost(uint32, tag = "1")]
    pub flags: u32,
    #[prost(uint32, tag = "2")]
    pub hi: u32,
    #[prost(uint32, tag = "3")]
    pub lo: u32,
    #[prost(uint32, tag = "4")]
    pub mid: u32,
}
