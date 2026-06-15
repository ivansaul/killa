use std::sync::LazyLock;

use scraper::Selector;

pub static ROW_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("tr.odd, tr.even").unwrap());

pub static FR_WRD_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".FrWrd").unwrap());

pub static TO_WRD_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".ToWrd").unwrap());

pub static FR_EX_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".FrEx").unwrap());

pub static TO_EX_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".ToEx").unwrap());

pub static STRONG_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("strong").unwrap());

pub static POS_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".POS2").unwrap());

pub static TD_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td").unwrap());
