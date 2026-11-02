use core::fmt;
use std::{error::Error, str::FromStr};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use super::{
    compoent::{exchange::ParamsExchange, search::ParamsSearch},
    paging::params::ParamsPaging,
};

/*
    b0    'bound',
    b10    'bound_today_same',
    b11    'bound_today_low_up',
    b12    'bound_today_low_downing',
    v0    'volume'


    f   'foot',
    k   'knee',
    s   'shoulder',
    h   'head',
*/
