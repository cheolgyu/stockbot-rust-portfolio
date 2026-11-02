> 원본 저장소와 개발 이력을 보고 싶으시면, 이력서의 연락처로 GitHub 초대에 사용할 이메일 주소를 알려 주세요. 확인 후 초대하겠습니다.

# Stockbot Rust | 수집·SQL·배치 운영 화면·AWS 빌드와 배포

국내 주가와 거래량을 모아 여러 해의 가격 흐름·기간별 통계를 같은 기준으로 비교하는 개인 리서치 도구입니다. **수집 → Rust Job/Step → PostgreSQL 가공·집계 → Actix API → Yew/Rust WASM 조회·운영 화면**을 연결했습니다.

이 저장소는 원본 `cheolgyu/stockbot-rust`의 **main 18c1f8e**를 복사해 공개 검토용으로 정리한 소스 스냅샷입니다. 운영 식별값·주소·자격증명 참조는 예시 값으로 바꿨고, 원본의 PR·Actions 실행 기록·전체 개발 이력은 가져오지 않았습니다. 원본은 비공개이며 위 안내에 따라 검토 초대를 요청할 수 있습니다.

## 먼저 볼 구현

| 경험 | 해결한 문제와 바로 볼 코드 |
| --- | --- |
| 수집·웹 스크래핑 | [Naver HTTP 수집](batch/src/downloader/naver/naver_download.rs#L15-L46) → [날짜·가격 응답 파싱](batch/src/downloader/naver/parer_file.rs#L16-L73), [KRX OTP→Excel](batch/src/downloader/krx/krx_download.rs#L53-L110), 별도 [HTML 뉴스 파서](batch/src/job/input_news/naver/scraping.rs#L93-L185) |
| 비동기 처리 | [Tokio 작업·async_channel::bounded(tx/rx) 생산자·소비자](batch/src/job/bound/step_bound_upsert.rs#L34-L106) → [계산·DB 저장 소비자](batch/src/job/bound/step_bound_upsert.rs#L114-L154) |
| 시계열·분석 SQL | [거래일 RANK](batch/init/init-2-tick.x_tick.sql#L12-L42), [종목별 LAG](batch/init/init-8-hist.price_chg.sql#L33-L74), [월·주·분기 UNION ALL·순위](batch/init/init-7-agg_vol.sql#L57-L109), [CTE·윈도 집계·반복 빈도](batch/init/init-7-agg_vol.sql#L134-L241) |
| 테이블·조회 설계 | [종목·시세 복합키·날짜 파티션](batch/init/init-1.sql#L9-L128), [bound 스키마](batch/init/init-2-bound.sql#L14-L47), [실행이력 FK](batch/init/init-0-system.sql#L2-L25) |
| 배치 운영 조회 | [실행이력 저장](batch/src/utils/batch/model.rs#L7-L52) → [Actix 로그 조회](site-api/src/handler/system.rs#L57-L85) → [WASM Job/Step 이력 표](site/src/components/table/tb_batch_log.rs#L152-L200) |
| API·모듈 | 9개 workspace crate, [공유 BatchParams·utoipa 계약](batch_model/src/app_type/batch_run_type.rs#L1-L44), [OpenAPI·Swagger UI](site-api/src/main.rs#L55-L125) |
| GitHub Actions CI/CD | [API ARM64→ECR→S3→CodeDeploy](.github/workflows/site-api.yml#L42-L86), [배치 이미지](.github/workflows/batch.yml), [스케줄러 배포](.github/workflows/batch_exe.yml), [WASM 정적 배포](.github/workflows/site.yml) |

## 시스템을 더 읽기

- [SQL·테이블 구조와 데이터 흐름](docs/SQL_AND_SCHEMA.md): 윈도 함수·다단계 집계·복합키·FK·파티션·조회 모델.
- [수집·웹 스크래핑과 배치 운영 조회](docs/COLLECTION_AND_OPERATIONS.md): 입력 형식·재수집·생산자/소비자·실행이력·계획·heartbeat.
- [GitHub Actions CI/CD와 ARM64 공통 빌드](docs/CI_CD.md): 공통 기반과 서비스별 확장, 산출물별 배포 경로.
- [원본·공개 사본 비교 기록](PUBLIC_SOURCE_PROVENANCE.json): 기준 커밋, 파일별 원본/공개 해시와 공개용 변환.

## 원본과 공개 사본의 범위

원래 Rust 구현은 2022-2025년 작업입니다. 2026년 AI 에이전트 보조 Axum·React 후속 작업은 원본의 다른 브랜치이며 이 main 사본과 구분합니다. 공개 사본의 GitHub Actions 실행은 비활성화했고 workflow·Docker·배포 스크립트는 설계 검토용으로 보존했습니다. 이 사본에서 AWS 배포를 실행한 결과는 없습니다.

뉴스 HTML 모듈은 구현이 있지만 기본 배치에 등록되지 않았고 소비자 await 누락도 남았습니다. 날짜 파티션의 연말 경계·LAG 증분 첫 행·기존 bound 꼬리 재계산은 보완 과제입니다. 이 구조는 개인 분석 도구이며 투자 성과나 대규모 서비스 운영 실적을 뜻하지 않습니다.
