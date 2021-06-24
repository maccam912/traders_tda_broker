use traders::{broker::Broker, Rational64};

pub struct TdaBroker {}

impl Broker for TdaBroker {
    fn new() -> Self {
        todo!()
    }

    fn get_cash(&self) -> Rational64 {
        todo!()
    }

    fn get_positions(&self) -> &std::collections::HashSet<traders::broker::Position> {
        todo!()
    }

    fn get_orders(&self) -> &[traders::broker::Order] {
        todo!()
    }

    fn new_order(&mut self, o: traders::broker::Order) {
        todo!()
    }
}
