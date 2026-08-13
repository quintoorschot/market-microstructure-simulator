use core::fmt;

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub price: i64,
    pub quantity: u64,
    pub side: Side,
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