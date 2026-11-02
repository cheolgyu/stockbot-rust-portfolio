use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};

pub const DEFAULT_DESC_VAL: bool = false;

const DESC: &str = "DESC";
const ASC: &str = "ASC";

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams, ToSchema)]
// #[into_params(names("page", "size", "sort", "desc"))]
#[into_params(parameter_in = Query)]
pub struct ParamPaging {
    #[schema(example = 1)]
    #[serde(deserialize_with = "de_from_str")]
    pub page: usize,
    #[schema(example = 10)]
    #[serde(deserialize_with = "de_from_str")]
    pub size: usize,
    #[schema(example = "code_id")]
    pub sort: String,
    #[serde(deserialize_with = "de_from_str_bool")]
    #[serde(serialize_with = "se_from_str")]
    #[schema(example = true)]
    pub desc: bool,
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

impl ParamPaging {
    pub fn page(&mut self, num: usize) {
        self.page = num.into();
    }
}

impl Default for ParamPaging {
    fn default() -> Self {
        Self {
            page: 1,
            size: 20,
            sort: "".to_string(),
            desc: DEFAULT_DESC_VAL,
        }
    }
}

impl ParamPaging {
    pub fn format_get(&self) -> String {
        let p = serde_urlencoded::to_string(self).unwrap();
        let url: String = format!("?{}", p);

        url
    }

    pub fn init_desc(&mut self) {
        self.desc = DEFAULT_DESC_VAL;
    }

    pub fn set_page_1(&mut self) {
        self.page(1)
    }

    pub fn set_page(&mut self, v: usize) {
        self.page(v.into())
    }

    pub fn set_sort(&mut self, v: String) {
        self.page(1);
        if self.sort == v {
            self.desc = !self.desc;
        } else {
            self.sort = v;
            self.desc = DEFAULT_DESC_VAL;
        }
    }

    pub fn sort(&self) -> String {
        self.sort.clone()
    }

    pub fn limit(&self) -> i32 {
        self.size.try_into().unwrap()
    }

    pub fn offset(&self) -> i32 {
        let p: i32 = self.page.try_into().unwrap();

        let s: i32 = self.size.try_into().unwrap();
        ((p - 1) * s) as i32
    }
    pub fn desc(&self) -> &str {
        if self.desc {
            DESC
        } else {
            ASC
        }
    }
}
