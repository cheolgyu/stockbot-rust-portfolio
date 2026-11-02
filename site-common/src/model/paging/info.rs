use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::params::ParamsPaging;

pub const HORIZONTAL_ELLIPSIS: usize = 3;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct Info {
    pub cur: usize,
    pub max: usize,
    pub total_cnt: usize,
    pub size: usize,
    pub paging: Vec<usize>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Default, Copy, Eq)]
pub struct PageNumber(pub usize);

impl Info {
    pub fn from(v: &ParamsPaging, total_cnt: i64) -> Self {
        let mut item = Self {
            cur: v.page,
            max: 0,
            total_cnt: total_cnt as usize,
            size: v.size,
            paging: vec![],
        };
        item.set_max();
        item
    }
    pub fn set_max(&mut self) {
        if self.total_cnt > 0 {
            let total_cnt: usize = self.total_cnt;
            let size: usize = self.size;

            println!("{:?}", self);

            let mut m = total_cnt / size;
            if total_cnt % size != 0 {
                m = m + 1;
            }
            self.max = m;

            let mut vec: Vec<usize> = vec![];
            for i in 1..m {
                vec.push(i);
            }

            self.paging = self.get_cur_show_page_numbers();
        }
    }

    pub fn get_cur_prev(&self) -> usize {
        let i = self.cur - 1;

        if i <= 1 {
            1
        } else {
            i
        }
    }

    pub fn get_cur_next(&self) -> usize {
        let i = self.cur + 1;

        if i >= self.max {
            self.cur
        } else {
            i
        }
    }

    pub fn get_cur_show_page_numbers(&self) -> Vec<usize> {
        let cur = self.cur;
        let mut res2: Vec<usize> = vec![];

        if !cur.checked_sub(HORIZONTAL_ELLIPSIS).is_none() {
            let start = cur - HORIZONTAL_ELLIPSIS;
            for x in start..cur {
                if x > 0 {
                    res2.push(x);
                }
            }
        } else {
            for x in 1..cur {
                res2.push(x);
            }
        }

        for x in cur..=cur + HORIZONTAL_ELLIPSIS {
            if x <= self.max {
                res2.push(x);
            }
        }

        res2
    }
}
