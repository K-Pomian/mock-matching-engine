use rust_decimal::Decimal;

use super::proto_decimal::ProtoDecimal;

impl From<Decimal> for ProtoDecimal {
    fn from(value: Decimal) -> Self {
        let unpacked_decimal = value.unpack();
        Self {
            negative: unpacked_decimal.negative,
            scale: unpacked_decimal.scale,
            hi: unpacked_decimal.hi,
            mid: unpacked_decimal.mid,
            lo: unpacked_decimal.lo,
        }
    }
}

impl Into<Decimal> for ProtoDecimal {
    fn into(self) -> Decimal {
        Decimal::from_parts(self.lo, self.mid, self.hi, self.negative, self.scale)
    }
}
