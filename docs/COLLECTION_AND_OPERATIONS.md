# 수집·웹 스크래핑과 배치 운영 조회

## 1. 입력 형식별 수집과 내부 모델 변환

기본 배치는 [Naver HTTP 시세 요청](../batch/src/downloader/naver/naver_download.rs#L15-L46)을 파일로 보관하고 [날짜·가격·거래량을 AddPrice로 파싱](../batch/src/downloader/naver/parer_file.rs#L16-L73)합니다. JSON 계열 응답의 인용부호·빈 값 형식을 정리하며, 종목별 최종 저장일과 upsert로 재수집을 다룹니다.

KRX는 [OTP 발급 → 파일 POST](../batch/src/downloader/krx/krx_download.rs#L53-L110)를 공통 수집기로 나누고 [Excel Sheet1을 파싱해 종목 상태와 연결](../batch/src/job/input_data/setp_code_state.rs#L42-L111)합니다. 네트워크 I/O·외부 형식·내부 저장 모델의 경계를 나눈 경험입니다.

뉴스에는 [HTML news_area·date·media·title·link 선택자 파서](../batch/src/job/input_news/naver/scraping.rs#L93-L185)와 날짜 범위·페이지 반복을 구현했습니다. 다만 기본 main 미등록과 소비자 `.await` 누락으로 현재 운영 수집 완료 기능에는 포함하지 않습니다.

## 2. 생산자·소비자와 작업 완료

[가격 파일 처리 명령과 여러 수신 작업](../batch/src/job/input_data/setp_price_naver.rs#L86-L191)을 Tokio 작업과 async_channel::bounded(tx/rx)로 연결합니다. bound는 [용량 1에서 send().await로 생산 대기](../batch/src/job/bound/step_bound_upsert.rs#L34-L106)하고 소비자가 계산·저장을 맡습니다. 소비자 JoinHandle을 모아 완료를 기다립니다. 별도 spawn으로 만든 가격 송신까지 전역 역압·메모리 제한이라고 설명하지 않습니다.

## 3. 실행이력 저장 → API → Rust/WASM 운영 화면

별도 ELK/Loki 같은 중앙 로그서버를 둔 구조는 아닙니다. [system.batch와 batch_step FK](../batch/init/init-0-system.sql#L2-L25)로 실행 부모와 Job/Step 기록을 나누고 [공통 모델의 시각·소요시간 INSERT](../batch/src/utils/batch/model.rs#L7-L52)를 실행 절차에서 사용합니다. Job/Step 시작·종료는 append 형태이고 Batch 부모의 완료 시각은 갱신합니다.

[이력 JOIN·정렬·페이지 조회 DAO](../site-api/src/dao/system_dao.rs#L30-L84) → [GET /system/logs/batch](../site-api/src/handler/system.rs#L57-L85) → [배치·Job·Step·소요시간·시작/종료 표](../site/src/components/table/tb_batch_log.rs#L152-L200)로 이어집니다. [운영 페이지는 실행계획과 이력 표를 함께 표시](../site/src/pages/system.rs#L25-L30)합니다.

`tracing`·Actix Logger는 프로세스 로그를 맡습니다. 원본의 오류 반환·unwrap 경로는 종료 기록을 남기지 못할 수 있어 실패 완료·재시작 보장으로 확대하지 않습니다.

## 4. 실행계획과 스케줄러 상태

[START/EXIT 계획의 생성·갱신·조회](../site-api/src/dao/cmd_dao.rs#L15-L92)와 API를 연결했습니다. 스케줄러는 평일·시각·당일 중복 조건을 확인하고 [heartbeat 시각을 system.info에 갱신](../batch_exe/src/kr/timer.rs#L111-L123)합니다. 화면에서 마지막 갱신 시각을 조회할 수 있으며 이 값이 실제 전체 배치 성공의 증명은 아닙니다.
