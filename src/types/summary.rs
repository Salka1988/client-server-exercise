use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, AtomicUsize};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CurrencyEnum {
    DOT,
    ROT,
    NONE
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Trend {
    RISING,
    FALLING,
    NONE
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub values: VecDeque<f32>,
    pub short_name: CurrencyEnum,
    pub name: String,
    pub lowest_price: f32,
    pub highest_price: f32,
    pub average_price: f32,
    pub trend: Trend,
}

impl Currency {
    pub fn new(
        short_name: CurrencyEnum,
        name: String,
    ) -> Self {
        Currency {
            values: VecDeque::new(),
            short_name,
            name,
            lowest_price: 0.000000,
            highest_price: 0.000000,
            average_price: 0.000000,
            trend: Trend::NONE
        }
    }

    pub fn update(&mut self) {

        let mut range = rand::thread_rng().gen_range(0.000000, 1.500000) as f32;

        if self.lowest_price == 0.000000 && self.highest_price == 0.000000 {
            self.lowest_price = range;
            self.highest_price = range;
            self.average_price = range;
        }

        let mut len = self.values.len();

        if len == 100 {
            self.values.pop_front();
            self.values.push_back(range);
        } else {
            self.values.push_back(range);
        }

        if self.lowest_price > range {
            self.lowest_price = range;
        }

        if self.highest_price < range {
            self.highest_price = range;
        }

        if len == 2 {
            self.average_price =  (self.values[len - 2] + self.values[len - 1]) / 2.0;
        }

        if len > 2 {
            self.average_price =
                (self.values[len - 3] + self.values[len - 2] + self.values[len - 1]) / 3.0;
        }

    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
   pub currencies_vec:  Vec<Arc<RwLock<Currency>>>
}

impl Currencies {
    pub fn new() -> Self {
        let mut currencies: Vec<Arc<RwLock<Currency>>> =
            vec![
                Arc::new(RwLock::new(Currency::new(CurrencyEnum::DOT, String::from("Polkadot")))),
                Arc::new(RwLock::new(Currency::new(CurrencyEnum::ROT, String::from("Rotor"))))
            ];
        Currencies { currencies_vec: currencies }
    }
}
