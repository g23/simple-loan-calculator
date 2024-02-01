use Compounding::*;

use crate::log;

#[derive(Debug, PartialEq)]
pub enum Compounding {
    Daily,
    Monthly,
    Yearly,
}

impl From<String> for Compounding {
    fn from(value: String) -> Self {
        use Compounding::*;
        if value == "daily" {
            Daily
        } else if value == "monthly" {
            Monthly
        } else if value == "yearly" {
            Yearly
        } else {
            Daily
        }
    }
}

#[derive(Debug)]
pub struct Loan {
    pub amount: f64,
    pub rate: f64,
    pub compounding: Compounding,
    pub payment: f64,
    pub payday: i32,
}

pub fn round2(amount: f64) -> f64 {
    (100.0 * amount).round() / 100.0
}

impl Loan {
    pub fn new<S: ToString>(
        amount: f64,
        apr: f64,
        compounding_str: S,
        payment: f64,
        payday: i32,
    ) -> Self {
        let rate = 0.01 * apr;
        let compounding = compounding_str.to_string().into();
        let payday = payday.max(1);
        Self {
            amount,
            rate,
            compounding,
            payment,
            payday,
        }
    }

    pub fn is_valid_pay_schedule(&self) -> bool {
        // need to check that payment, made every payday # of days
        // is more than the loan increases
        let mut amount = self.amount;
        let daily_rate = self.rate / 365.0;
        let monthly_rate = self.rate / 12.0;
        let yearly_rate = self.rate;
        // start day at 1 so don't immediately get yearly interest
        for day in 1..self.payday + 1 {
            if self.compounding == Daily {
                amount += amount * daily_rate;
            } else if self.compounding == Monthly && day % 30 == 0 {
                amount += amount * monthly_rate;
            } else if self.compounding == Yearly && day % 365 == 0 {
                amount += amount * yearly_rate;
            }
        }
        // do we pay more than the interest accures?
        let interest_accrued = amount - self.amount;
        self.payment > interest_accrued
    }

    pub fn run_until_done(&self) -> Result<Snapshot, ()> {
        if !self.is_valid_pay_schedule() {
            // if we run this, it'd be an infinite loop...
            return Err(());
        }

        // figure rates depending on how the loan compounds
        let daily_rate = self.rate / 365.0;
        let monthly_rate = self.rate / 12.0;
        let yearly_rate = self.rate;

        let mut amount = self.amount;

        let mut day = 0;

        let payment = self.payment;
        let payday = self.payday;
        let mut total_paid = 0.0;

        log!("Starting the big calculation");
        log!(
            "amount = {amount}, compounding = {c:?}",
            c = self.compounding
        );
        log!("paying: {payment} every {payday} days");

        while amount > 0.001 {
            // could probably do 0.0, but don't want floating shenanigans
            day += 1;
            // add the interest
            if self.compounding == Daily {
                amount += amount * daily_rate;
            } else if self.compounding == Monthly && day % 30 == 0 {
                amount += amount * monthly_rate;
            } else if self.compounding == Yearly && day % 365 == 0 {
                amount += amount * yearly_rate;
            }
            // make it a nice number
            amount = round2(amount);
            // now see if it's time for a payment
            if day % payday == 0 {
                // and make the payment
                // if payment is more than the amount
                if payment > amount {
                    // then pay it off completely
                    total_paid += amount;
                    amount = 0.0;
                } else {
                    // else just make the payment
                    total_paid += payment;
                    amount -= payment;
                }
            }
        }
        Ok(Snapshot::new(
            self.amount,
            self.rate,
            self.payment,
            self.payday,
            total_paid - self.amount,
            day,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub amount: f64,
    pub apr: f64,
    pub payment: f64,
    pub every: i32,
    pub human_time: String,
    pub interest: f64,
}

fn days_to_human_time(mut days: i32) -> String {
    let years = days / 365;
    days = days - years * 365;
    let months = days / 30;
    days = days - months * 30;
    let mut human_time = String::new();
    if years > 0 {
        human_time.push_str(&format!("{years} years ")[..]);
    }
    if months > 0 {
        human_time.push_str(&format!("{months} months ")[..]);
    }
    human_time.push_str(&format!("{days} days")[..]);
    human_time
}

impl Snapshot {
    pub fn new(amount: f64, apr: f64, payment: f64, every: i32, interest: f64, days: i32) -> Self {
        let human_time = days_to_human_time(days);
        let interest = round2(interest);
        Self {
            amount,
            apr,
            payment,
            every,
            human_time,
            interest,
        }
    }
}
