use crate::{
    job::{
        bound::utils::{get_price_after_p1x, hist_code_with_start_dt_list},
        input_data::file::{fm, FileInfo},
    },
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_channel::Receiver;
use async_trait::async_trait;
use batch_model::{app_type::price_type::PriceType, bound::AddBound, price::PriceWithConvertDate};
use sqlx::{Pool, Postgres};
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    task::JoinHandle,
};
use tracing::log::info;

use crate::job::bound::hist_make_bound::MakeBound;

use super::code_start_dt::CodeStartDt;
/*


1995-05-02 ~ 2025-09-25 바운드 csv 파일크기 1.8 기가 바이트
한번에 쿼리문 만들어서 전송하니 오류발생하여 copy 로 대체

파일 만들고 컨테이너 접속해서 psql 연결하고 copy 문날리기

rm -rf ./bound.csv

docker cp ./file.sql stockbot_db_dev:/
psql -U postgres -d stockbot-dev

drop INDEX  hist.hist_bound_p1x_asc_idx  ;
drop INDEX  hist.hist_bound_p1x_desc_idx ;
drop INDEX  hist.hist_bound_p2x_asc_idx  ;
drop INDEX hist.hist_bound_p2x_desc_idx ;


COPY hist.bound(code_id, line, p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b ) FROM '/bound.csv' WITH (FORMAT CSV, HEADER);
select count(*) from hist.bound ;--23917656 6초

CREATE INDEX IF NOT EXISTS hist_bound_p1x_asc_idx ON hist.bound (p1x asc);
CREATE INDEX IF NOT EXISTS hist_bound_p1x_desc_idx ON hist.bound (p1x desc);
CREATE INDEX IF NOT EXISTS hist_bound_p2x_asc_idx ON hist.bound (p2x asc);
CREATE INDEX IF NOT EXISTS hist_bound_p2x_desc_idx ON hist.bound (p2x desc);
select count(*) from hist.bound ; -- 2.3초 ,23923126



*/
pub struct StepBoundCsvCopy {
    pub batch_id: i64,
    pub code: Vec<CodeStartDt>,
}

type ChannelDataType = (i64, Vec<PriceWithConvertDate>, i64, PriceType);
#[async_trait]
impl Step for StepBoundCsvCopy {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let code = hist_code_with_start_dt_list(pool).await.unwrap();
        let init = "code_id, line, p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b \n ";

        self.code = code;
        /////////
        let fi: FileInfo = FileInfo::new();
        let file_path = get_fnm(fi.kr_bound_dir, self.batch_id);
        fm::mkdir_without_fnm(file_path.clone());
        let mut file = get_file(file_path, false).await;
        file.write_all(init.as_bytes()).await.unwrap();
        file.flush().await.unwrap();

        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        let batch_id = self.batch_id;
        ////////////////////////////////////////////////////
        info!("StepHistInsert process start");
        let items = self.code.clone();
        let loop_size: usize = items.len() * 4;
        //let cap = 16;
        let cap = 1;
        let (tx, rx) = async_channel::bounded::<ChannelDataType>(cap);
        /*
            ======================================
                        sender
            ======================================
        */
        let r = rx.clone();
        let pool1 = pool.clone();
        let s1 = tokio::spawn(async move {
            for item in items.iter() {
                let code_id = item.id.clone();
                let pool2 = pool1.clone();

                for &line in PriceType::iterator() {
                    let p1x = item.get_dt(line);
                    let pool3 = pool2.clone();
                    let pool4 = pool3.clone();

                    let list: Vec<PriceWithConvertDate> = get_price_after_p1x(pool3, code_id, p1x)
                        .await
                        .expect(&format!(
                            "get_price_after_p1x 오류 code_id={}, p1x={:?}",
                            code_id, p1x
                        ));
                    let len = &list.len();

                    match tx.send((batch_id, list.clone(), code_id, line)).await {
                        Ok(_) => {
                            //info!("send: code_id={code_id},line={line},len={}", len);
                        }
                        Err(err) => {
                            println!("작업종료send={:?}", err);
                            if tx.is_empty() {
                                tx.close();
                            }
                        }
                    };
                }
            }
        });
        /*
            ======================================
                        recviver
            ======================================
        */
        let mut recv_size: usize = loop_size / 100;
        if recv_size < 1 {
            recv_size = 1;
        }
        let pool_size = 10;

        let mut recv_manager: Vec<JoinHandle<()>> = Vec::new();

        for n in 0..recv_size {
            info!("rx make ={}", n);
            let rx_name = format!("{}", n);

            recv_manager.push(recv(n, rx.clone(), &pool.clone()).await);
        }

        for r in recv_manager {
            r.await
                .map_err(|err| {
                    info!("무슨 오류지? {:?}", err);
                    panic!("{}", err)
                })
                .unwrap();
        }

        s1.await.unwrap();

        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
pub async fn recv(
    rx_name: usize,
    rx: Receiver<ChannelDataType>,
    pool: &Pool<Postgres>,
) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        loop {
            let res = rx.recv().await;
            match res {
                Ok((batch_id, list, code_id, line)) => {
                    info!(
                        "recv({rx_name}): code_id={code_id},line={line},len={}",
                        list.len()
                    );
                    if list.len() > 2 {
                        let make_bound = MakeBound {
                            code_id,
                            line,
                            list,
                        };

                        let list_bound: Vec<AddBound> = make_bound.run();
                        let size = list_bound.len();
                        if size > 0 {
                            let fi: FileInfo = FileInfo::new();

                            let items = list_bound.iter().map(|item: &AddBound| {
                                let code_id = item.code_id;
                                let line = item.line;
                                let p1x = item.p1x;
                                let p1y = item.p1y;
                                let p2x = item.p2x;
                                let p2y = item.p2y;
                                let direction = item.direction;
                                let x_cnt = item.x_cnt;
                                let y_cnt = item.y_cnt;
                                let change_rate = item.change_rate.round_dp(2);
                                let ymxb_p3y = item.ymxb_p3y.round_dp(2);
                                let ymxb_m = item.ymxb_m.round_dp(2);
                                let ymxb_b = item.ymxb_b.round_dp(2);

                                format!("{code_id},{line},{p1x},{p1y},{p2x},{p2y},{direction},{x_cnt},{y_cnt},{change_rate},{ymxb_p3y},{ymxb_m},{ymxb_b} \n")
                            }).collect::<String>();

                            let file_path = get_fnm(fi.kr_bound_dir, batch_id);
                            let mut file = get_file(file_path, true).await;
                            file.write_all(items.as_bytes()).await.unwrap();
                            file.flush().await.unwrap();
                        }
                    }
                }
                //Received because the channel is empty and closed.
                Err(err) => {
                    //println!("작업종료recv={:?}", err);
                    if rx.is_empty() {
                        rx.close();
                        break;
                    }
                }
            }
        }
    })
}

fn get_fnm(kr_bound_dir: String, batch_id: i64) -> String {
    let file_path = format!("{}/{}/bound.csv", kr_bound_dir, batch_id);
    file_path
}

fn get_fnm_finish(kr_bound_dir: String, batch_id: i64) -> String {
    let file_path = format!("{}/{}_finish/bound.csv", kr_bound_dir, batch_id);
    file_path
}

async fn get_file(file_path: String, append: bool) -> File {
    if append {
        OpenOptions::new()
            .write(true)
            .append(true) // Optional: Truncate the file if it exists
            .open(file_path)
            .await
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Optional: Truncate the file if it exists
            .open(file_path)
            .await
            .unwrap()
    }
}

mod tests {
    use tokio::{
        fs::{File, OpenOptions},
        io::{
            self, AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter,
        },
    };

    use crate::job::input_data::file::FileInfo;

    #[tokio::test]
    async fn test_file() {
        let fi: FileInfo = FileInfo::new();
        let file_path = format!("{}/file.sql", fi.kr_bound_dir);
        println!("file_path={:?}", file_path);
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)
            .await
            .unwrap();
        //file last =
        //(501,CLOSE,2025-09-22,7790,2025-09-25,8230,INCREASE,3,440,5.648267008985879332477535300,7936.6666666666666666666667,146.66666666666666666666666667,-1030756.6666666666666666666667),
        //  f.seek(io::SeekFrom::End(20)).await.unwrap();

        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        // read a line into buffer
        reader.read_line(&mut buffer).await.unwrap();

        println!("buffer={}", buffer);

        // f.flush().await.unwrap();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_file2() {
        let fi: FileInfo = FileInfo::new();
        let file_path = format!("{}/file copy.sql", fi.kr_bound_dir);
        let file_path2 = format!("{}/new_file copy.sql", fi.kr_bound_dir);
        println!("file_path={:?}", file_path);
        ///////////////////
        ///
        let mut contents = tokio::fs::read_to_string(file_path).await.unwrap();

        println!("File has {} lines.", contents.lines().count());
        if contents.ends_with(",") {
            println!(" contents.ends_wit ,");
        } else {
            println!(" contents.ends_wit , not ");
        }

        let last_comma = contents.rfind(",").unwrap();

        contents.replace_range(last_comma.., ";");

        let mut file = BufWriter::new(File::create(file_path2).await.unwrap());

        file.write_all(contents.as_bytes()).await.unwrap();

        // Due to the BufWriter, the actual write and spawn_blocking
        // call happens when you flush.
        file.flush().await.unwrap();

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
