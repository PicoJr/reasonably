use break_infinity::Decimal;

fn format_decimal(decimal: Decimal, unit: &str) -> String {
    let stages: Vec<(Decimal, String)> = vec![
        (Decimal::new(1e3), "".to_string()),
        (Decimal::new(1e6), "Kilo".to_string()),
        (Decimal::new(1e9), "Mega".to_string()),
        (Decimal::new(1e12), "Giga".to_string()),
        (Decimal::new(1e15), "Tera".to_string()),
        (Decimal::new(1e18), "Peta".to_string()),
        (Decimal::new(1e21), "Exa".to_string()),
    ];
    for (limit, symbol) in stages {
        if decimal.abs() < limit {
            return format!(
                "{} {}{}",
                (decimal / (limit / Decimal::new(1e3))).to_fixed(2),
                symbol,
                unit
            );
        }
    }
    format!("{} {}", decimal.to_precision(2), unit)
}

pub(crate) fn format_decimal_devs(decimal: Decimal) -> String {
    format_decimal(decimal, "devs")
}

pub(crate) fn format_decimal_bugs(decimal: Decimal) -> String {
    format_decimal(decimal, "bugs")
}

pub(crate) fn format_decimal_hrs(decimal: Decimal) -> String {
    format_decimal(decimal, "hrs")
}

pub(crate) fn format_decimal_pms(decimal: Decimal) -> String {
    format_decimal(decimal, "pms")
}

pub(crate) fn format_decimal_features(decimal: Decimal) -> String {
    format_decimal(decimal, "features")
}

pub(crate) fn format_decimal_loc(decimal: Decimal) -> String {
    // Linux Kernel 5.11 approximately has 30 millions lines of code cf https://en.wikipedia.org/wiki/Linux_kernel
    let linux_kernel_loc: Decimal = Decimal::new(30e6);
    let stages: Vec<(Decimal, Decimal, String)> = vec![
        (Decimal::new(1e3), Decimal::new(1e3), "loc".to_string()),
        (Decimal::new(1e6), Decimal::new(1e6), "Kiloloc".to_string()),
        (linux_kernel_loc, Decimal::new(1e9), "Megaloc".to_string()),
        (
            linux_kernel_loc * Decimal::new(1e3),
            linux_kernel_loc * Decimal::new(1e3),
            "Linux Kernels (LK)".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e6),
            linux_kernel_loc * Decimal::new(1e6),
            "KiloLK".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e9),
            linux_kernel_loc * Decimal::new(1e9),
            "MegaLK".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e12),
            linux_kernel_loc * Decimal::new(1e12),
            "GigaLK".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e15),
            linux_kernel_loc * Decimal::new(1e15),
            "TeraLK".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e18),
            linux_kernel_loc * Decimal::new(1e18),
            "PetaLK".to_string(),
        ),
        (
            linux_kernel_loc * Decimal::new(1e21),
            linux_kernel_loc * Decimal::new(1e21),
            "ExaLK".to_string(),
        ),
    ];
    for (limit, size, unit) in stages {
        if decimal.abs() < limit {
            return format!(
                "{} {}",
                (decimal / (size / Decimal::new(1e3))).to_fixed(2),
                unit
            );
        }
    }
    format!("{} LK", (decimal / linux_kernel_loc).to_precision(2))
}
