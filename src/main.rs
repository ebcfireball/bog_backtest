use std::collections::HashMap;

use csv::{Error, ReaderBuilder};
use serde::Deserialize;
fn main() -> Result<(), Error> {
    const CSV_FILE: &'static str = "14oct302019.csv";
    let mut reader = ReaderBuilder::new().from_path(CSV_FILE).unwrap();

    let mut orders: HashMap<u64, Message> = HashMap::new();
    let og_cash =10000000;
    let mut cash: isize = 10000000;
    let mut holds_stock = true;
    let mut is_short = false;
    let mut lastexec: u32 = 0;
    let mut trend = false;
    //trend true is up trend trend false is down trend


    //first for loop is naive micro trend following
    //for res in reader.deserialize() {
        //let row: Message = res?;
        //let typ = row.typ.unwrap() as char;
        //if typ == 'A' || typ == 'F' {
            //orders.insert(row.orrf.unwrap(), row);
        //} else if typ == 'X' {
            //let order = orders.get_mut(&row.orrf.unwrap()).unwrap();
            //order.shares = Some(order.shares.unwrap() - row.cancelled_shares.unwrap());
        //} else if typ == 'D' {
            //orders.remove(&row.orrf.unwrap());
        //} else if typ == 'U' {
            //let mut del = orders.remove(&row.orrf.unwrap()).unwrap();
            //del.shares = row.shares;
            //del.price = row.price;
            //orders.insert(row.new_orff.unwrap(), del);
        //} else if typ == 'E' || typ == 'C' {
            //let order: &mut Message = orders.get_mut(&row.orrf.unwrap()).unwrap();
            //order.shares = Some(order.shares.unwrap() - row.executed_shares.unwrap());
            //let currexec = {
                //if typ == 'E' {
                    //order.price.unwrap()
                //} else {
                    //row.executed_price.unwrap()
                //}
            //};
            //if currexec > lastexec {
                //if !holds_stock {
                    //if is_short {
                        //cash -= (row.ask.unwrap() * 2) as isize;
                        //is_short = false;
                    //} else {
                        //cash -= row.ask.unwrap() as isize;
                    //}
                    //holds_stock = true;
                //}
            //} else if currexec < lastexec {
                //if !is_short {
                    //if holds_stock {
                        //cash += (row.bid.unwrap() * 2) as isize;
                        //holds_stock = false;
                    //} else {
                        //cash += row.bid.unwrap() as isize;
                    //}
                    //is_short = true;
                //}
            //}
            //lastexec = currexec;
        //}
    //}

    //level 1 micro trend following
    for res in reader.deserialize() {
        let row: Message = res?;
        let typ = row.typ.unwrap() as char;
        if typ == 'A' || typ == 'F' {
            orders.insert(row.orrf.unwrap(), row);
        } else if typ == 'X' {
            let order = orders.get_mut(&row.orrf.unwrap()).unwrap();
            order.shares = Some(order.shares.unwrap() - row.cancelled_shares.unwrap());
        } else if typ == 'D' {
            orders.remove(&row.orrf.unwrap());
        } else if typ == 'U' {
            let mut del = orders.remove(&row.orrf.unwrap()).unwrap();
            del.shares = row.shares;
            del.price = row.price;
            orders.insert(row.new_orff.unwrap(), del);
        } else if typ == 'E' || typ == 'C' {
            let order: &mut Message = orders.get_mut(&row.orrf.unwrap()).unwrap();
            order.shares = Some(order.shares.unwrap() - row.executed_shares.unwrap());
            let currexec = {
                if typ == 'E' {
                    order.price.unwrap()
                } else {
                    row.executed_price.unwrap()
                }
            };
            if currexec > lastexec {
                trend = true;
                if row.ask.unwrap() == currexec {
                    if !holds_stock {
                        if is_short {
                            cash -= (row.ask.unwrap() * 2) as isize;
                            is_short = false;
                        } else {
                            cash -= row.ask.unwrap() as isize;
                        }
                        holds_stock = true;
                    }
                }
            } else if currexec < lastexec {
                trend = false;
                if row.bid.unwrap() == currexec {
                    if !is_short {
                        if holds_stock {
                            cash -= (row.bid.unwrap() * 2) as isize;
                            holds_stock = false;
                        } else {
                            cash -= row.bid.unwrap() as isize;
                        }
                        is_short = true;
                    }
                }
            } else if currexec == lastexec {
                if trend {
                    if row.ask.unwrap() == currexec {
                        if !holds_stock {
                            if is_short {
                                cash -= (row.ask.unwrap() * 2) as isize;
                                is_short = false;
                            } else {
                                cash -= row.ask.unwrap() as isize;
                            }
                            holds_stock = true;
                        }
                    }
                } else {
                    if row.bid.unwrap() == currexec {
                        if !is_short {
                            if holds_stock {
                                cash += (row.bid.unwrap() * 2) as isize;
                                holds_stock = false;
                            } else {
                                cash += row.bid.unwrap() as isize;
                            }
                            is_short = true;
                        }
                    }
                }
            }
            lastexec = currexec;
        }
    }


    print!("{} p/l, ",(cash-og_cash)/10000);
    print!("{cash} ending value");
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub typ: Option<u8>,
    pub timestamp: Option<u64>,
    pub orrf: Option<u64>,
    pub buy_sell: Option<u8>,
    pub shares: Option<u32>,
    pub price: Option<u32>,
    pub executed_shares: Option<u32>,
    pub executed_price: Option<u32>,
    pub new_orff: Option<u64>,
    pub cancelled_shares: Option<u32>,
    pub bid: Option<u32>,
    pub ask: Option<u32>,
    pub spread: Option<u32>,
    pub ask_depth: Option<u32>,
    pub bid_depth: Option<u32>,
    pub depth: Option<u32>,
}
