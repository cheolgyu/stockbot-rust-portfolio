use dotenvy_macro::dotenv;
use sqlx::types::chrono::NaiveDate;
use tracing::info;

// 20220520 => 2022-05-20
pub fn url_date_format(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

#[derive(Debug)]
pub struct FileInfo {
    pub data_dir: String,
    pub kr_company_dir: String,
    pub kr_company_detail_full: String,
    pub kr_company_state_full: String,
    pub kr_price_dir: String,
    pub kr_bound_dir: String,
}
impl FileInfo {
    pub fn new() -> Self {
        let item = Self {
            data_dir: dotenvy::var("DOWNLOAD_DIR").unwrap().to_string(),
            kr_company_dir: dotenvy::var("DOWNLOAD_KR_COMPANY").unwrap().to_string(),
            kr_company_detail_full: dotenvy::var("DOWNLOAD_KR_COMPANY_DETAIL")
                .unwrap()
                .to_string(),
            kr_company_state_full: dotenvy::var("DOWNLOAD_KR_COMPANY_STATE")
                .unwrap()
                .to_string(),
            kr_price_dir: dotenvy::var("DOWNLOAD_KR_PRICE").unwrap().to_string(),
            kr_bound_dir: dotenvy::var("DOWNLOAD_KR_BOUND").unwrap().to_string(),
        };

        //info!("{:?}", &item);

        item
    }
}

pub mod fm {
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn mkdir() {
        let fi: FileInfo = FileInfo::new();
        let mk_path = vec![
            fi.data_dir,
            fi.kr_company_dir,
            fi.kr_price_dir,
            fi.kr_bound_dir,
        ];

        for p in mk_path {
            let path = Path::new(&p);
            let display = path.to_str().unwrap();

            match fs::create_dir_all(&path) {
                Err(why) => panic!("경로 생성 불가 {}: {}", display, why),
                Ok(()) => info!("경로 생성 성공 {}", p),
            }
        }
    }

    pub fn mkdir_file_path(file_path: String) {
        let path = Path::new(&file_path);
        let display = path.to_str().unwrap();

        match fs::create_dir_all(&path) {
            Err(why) => panic!("경로 생성 불가 {}: {}", display, why),
            Ok(()) => info!("경로 생성 성공 {}", file_path),
        }
    }

    pub fn mkdir_price_batch_id(batch_id: i64) {
        let fi: FileInfo = FileInfo::new();
        let p = format!("{}/{}", fi.kr_price_dir, batch_id).to_string();

        let path = Path::new(&p);
        let display = path.to_str().unwrap();

        match fs::create_dir_all(&path) {
            Err(why) => panic!("경로 생성 불가 {}: {}", display, why),
            Ok(()) => info!("경로 생성 성공 {}", p),
        }
    }

    // a/b/c/d.xlse 입력시 a/b/c 폴더경로 생성
    pub fn mkdir_without_fnm(full_path: String) {
        let mut p = full_path;
        let pos: usize = p.rfind("/").unwrap();
        let t: String = p.drain(..pos).collect();
        let path = Path::new(&t);
        match fs::create_dir_all(&path) {
            Err(why) => panic!("경로 생성 불가 {}: {}", t, why),
            Ok(()) => {
                //println!("경로 생성 성공 {}", t)
            }
        }
    }

    pub fn write_all(file_path: &str, body_bytes: &[u8]) {
        mkdir_without_fnm(file_path.to_string());
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path).unwrap();
        }

        let path = Path::new(&file_path);
        let display = path.to_str().unwrap();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match OpenOptions::new().write(true).create(true).open(&path) {
            Err(why) => panic!("파일 생성불가 {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(body_bytes) {
            Err(why) => panic!("파일 쓰기불가 {}: {}", &display, why),
            Ok(_) => info!("파일 쓰기 {} ", file_path),
        }
    }

    pub fn write_all_2(file_path: &str, body_bytes: &[u8]) {
        mkdir_without_fnm(file_path.to_string());
        // if Path::new(&file_path).exists() {
        //     fs::remove_file(&file_path).unwrap();
        // }

        let path = Path::new(&file_path);
        let display = path.to_str().unwrap();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match OpenOptions::new().write(true).create(true).open(&path) {
            Err(why) => panic!("파일 생성불가 {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(body_bytes) {
            Err(why) => panic!("파일 쓰기불가 {}: {}", &display, why),
            Ok(_) => info!("파일 쓰기 {} ", file_path),
        }
    }
    use std::fs::File;
    use std::io::BufReader;

    use tracing::info;

    use super::FileInfo;
    pub fn open(file_path: String) -> BufReader<File> {
        let path = Path::new(&file_path);

        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
    }
}
