pgadmin4 에서 백업 파일 방법
1. 데이스베이스 백업하기 파티션 테이블 말고. 
2. 백업 타입을 plane (text) 말고 custom 으로 하기: restore 에서 plane 안읽힘. custom으로 하고 파일열떄 파일 깨졌다고 나오지만 그냥 열면됨. 꺠진라인 삭제 금지.
3. restore 오래걸림.

2025-09-25 데이터 기준
stockbot-dev-rust-v3.5.3.bakcup.sql  select count(*) from hist.price ; --11045205

psql -U postgres -d stockbot-dev
 pg_dump -Fc stockbot-dev > stockbot-dev.dump

pg_admin 미 사용, 이유: 새로나온 pg 18버전은 pg_dump 안됨.
백업 방법
- docker exec -t <container_name> pg_dump -U <dbuser> <db-name> > FILENAME.sql
- docker exec -t stockbot_db_dev pg_dump -U postgres  stockbot-dev > stockbot-dev-rust_v3.5.5.1_rate_by_date.dump
복원 방법, psql 사용 
docker cp ./stockbot-dev-rust_v3.5.4_with_bound.sql stockbot_db_dev:/stockbot-dev-rust_v3.5.4_with_bound.sql
docker exec -t stockbot_db_dev psql -U postgres -d rmdb  -f  ./stockbot-dev-rust_v3.5.4_with_bound.sql

2025-09-25 까지 데이터 수
select count(*) from hist.price; --11045205
select count(*) from hist.bound; --23923126 

