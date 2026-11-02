use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};

pub const DEFAULT_DESC_VAL: bool = false;

const DESC: &str = "DESC";
const ASC: &str = "ASC";

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams, ToSchema)]
// #[into_params(names("page", "size", "sort", "desc"))]
#[into_params(parameter_in = Query)]
pub struct ParamPagingTwoSort {
    #[schema(example = 1)]
    #[serde(deserialize_with = "de_from_str")]
    pub page: usize,
    #[schema(example = 10)]
    #[serde(deserialize_with = "de_from_str")]
    pub size: usize,
    #[schema(example = "code_id")]
    pub sort1: Option<String>,
    #[schema(example = "code_id")]
    pub sort2: Option<String>,
    #[serde(deserialize_with = "de_from_str_bool")]
    #[serde(serialize_with = "se_from_str")]
    #[schema(example = true)]
    pub desc1: bool,
    #[serde(deserialize_with = "de_from_str_bool")]
    #[serde(serialize_with = "se_from_str")]
    #[schema(example = true)]
    pub desc2: bool,

    /// 3번 클릭하면 소트 취소 되는 코드 필요
    ///
    ///
    #[serde(deserialize_with = "de_from_str")]
    pub sort1_cnt: usize,
    #[serde(deserialize_with = "de_from_str")]
    pub sort2_cnt: usize,
}

fn se_from_str<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(*v)

    //bool::from_str(&s).map_err(de::Error::custom)
}

fn de_from_str<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    s.parse::<usize>().map_err(de::Error::custom)
}

fn de_from_str_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    bool::from_str(&s).map_err(de::Error::custom)
}

impl ParamPagingTwoSort {
    pub fn page(&mut self, num: usize) {
        self.page = num.into();
    }

    pub fn default_sort(sort1: &str, desc1: bool) -> Self {
        Self {
            page: 1,
            size: 20,
            sort1: Some(sort1.to_owned()),
            desc1: desc1,
            sort2: None,
            desc2: DEFAULT_DESC_VAL,
            sort1_cnt: 0,
            sort2_cnt: 0,
        }
    }

    pub fn default_sort2(sort1: &str, desc1: bool, sort2: &str, desc2: bool) -> Self {
        Self {
            page: 1,
            size: 20,
            sort1: Some(sort1.to_owned()),
            desc1: desc1,
            sort2: Some(sort2.to_owned()),
            desc2: desc2,
            sort1_cnt: 0,
            sort2_cnt: 0,
        }
    }
}

impl Default for ParamPagingTwoSort {
    fn default() -> Self {
        Self {
            page: 1,
            size: 20,
            sort1: Some("dt".to_owned()),
            desc1: true,
            sort2: None,
            desc2: DEFAULT_DESC_VAL,
            sort1_cnt: 0,
            sort2_cnt: 0,
        }
    }
}

impl ParamPagingTwoSort {
    fn to_display(desc: bool) -> &'static str {
        match desc {
            true => "▲",
            false => "▼",
        }
    }
    fn to_display2(desc: bool) -> &'static str {
        match desc {
            true => "△",
            false => "▽",
        }
    }

    pub fn sort_val(&self, k: &str, v: &str) -> String {
        let s1 = &self.sort1;
        let d1 = &self.desc1;
        let s2 = &self.sort2;
        let d2 = &self.desc2;

        let is_s1 = s1.clone().is_some_and(|x| x == *k);
        let is_s2 = s2.clone().is_some_and(|x| x == *k);
        let sort_val = if is_s1 {
            format!(" {} {}", v, Self::to_display(*d1))
        } else if is_s2 {
            format!(" {} {}", v, Self::to_display2(*d2))
        } else {
            v.to_string()
        };

        sort_val
    }

    pub fn format_get(&self) -> String {
        let p = serde_urlencoded::to_string(self).unwrap();
        let url: String = format!("?{}", p);

        url
    }
    fn init_sort(&mut self) {
        self.sort1_cnt = 0;
        self.sort2_cnt = 0;
        self.sort1 = None;
        self.sort2 = None;
    }

    pub fn init_desc(&mut self) {
        self.desc1 = DEFAULT_DESC_VAL;
        self.desc2 = DEFAULT_DESC_VAL;
    }

    pub fn set_page_1(&mut self) {
        self.page(1)
    }

    pub fn set_page(&mut self, v: usize) {
        self.page(v.into())
    }

    pub fn set_sort_mutil(&mut self, v: String) {
        self.page(1);
        let cnt1 = self.sort1_cnt;
        let cnt2 = self.sort2_cnt;
        match &self.sort1 {
            Some(v1) => {
                if *v1 == v {
                    if cnt1 == 2 {
                        self.init_sort();
                    } else {
                        self.desc1 = !self.desc1;
                        self.sort1_cnt = self.sort1_cnt + 1
                    }
                } else {
                    match &self.sort2 {
                        Some(v2) => {
                            if *v2 == v {
                                if cnt2 == 2 {
                                    self.init_sort();
                                } else {
                                    self.desc2 = !self.desc2;
                                    self.sort2_cnt = self.sort2_cnt + 1
                                }
                            } else {
                                self.init_sort();
                            }
                        }
                        None => {
                            self.sort2 = Some(v);
                            self.desc2 = !self.desc2;
                            self.sort2_cnt = 1
                        }
                    }
                }
            }
            None => {
                self.sort1 = Some(v);
                self.desc1 = !self.desc1;
                self.sort1_cnt = cnt1 + 1
            }
        };
    }

    pub fn sql_print(&self) -> String {
        if self.sort1 == None {
            "".to_string()
        } else {
            if self.sort2 == None {
                format!(
                    " order by  {} {}",
                    self.sort1.clone().unwrap(),
                    self.desc1()
                )
            } else {
                format!(
                    " order by  {} {}, {} {}",
                    self.sort1.clone().unwrap(),
                    self.desc1(),
                    self.sort2.clone().unwrap(),
                    self.desc2()
                )
            }
        }
    }

    pub fn limit(&self) -> i32 {
        self.size.try_into().unwrap()
    }

    pub fn offset(&self) -> i32 {
        let p: i32 = self.page.try_into().unwrap();

        let s: i32 = self.size.try_into().unwrap();
        ((p - 1) * s) as i32
    }
    fn desc1(&self) -> &str {
        if self.desc1 {
            DESC
        } else {
            ASC
        }
    }
    fn desc2(&self) -> &str {
        if self.desc2 {
            DESC
        } else {
            ASC
        }
    }
}
