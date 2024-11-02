use break_infinity::Decimal;

pub(crate) fn format_decimal(decimal: Decimal) -> String {
    if decimal.abs() < Decimal::new(1000.0) {
        // small numbers -> 2 digits after decimal point
        format!("{:.2}", decimal.to_number())
    } else {
        format!("{}", decimal)
    }
}