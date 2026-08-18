use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq)]
pub struct Order {
    pub id: u64,
    pub price: i64,     // Price in ticks (e.g. 10050 = $100.50)
    pub quantity: u64,
    pub side: Side,
    pub timestamp: u64,
}

impl Order {

    fn price_decimal(&self) -> f64 {
        self.price as f64 / 100.0
    }

}

impl fmt::Display for Order {

    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {

        let side = match self.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        write!(
            f,
            "{} {} @ {:.2} (order_id={})",
            side,
            self.quantity,
            self.price_decimal(),
            self.id,
        )

    }

}