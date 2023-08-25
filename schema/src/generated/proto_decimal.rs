#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDecimal {
    #[prost(bool, tag = "1")]
    pub negative: bool,
    #[prost(uint32, tag = "2")]
    pub scale: u32,
    #[prost(uint32, tag = "3")]
    pub hi: u32,
    #[prost(uint32, tag = "4")]
    pub lo: u32,
    #[prost(uint32, tag = "5")]
    pub mid: u32,
}
