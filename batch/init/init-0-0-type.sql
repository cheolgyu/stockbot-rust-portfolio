/*
 
 
 스키마 생성
 
 
 */
CREATE SCHEMA IF NOT EXISTS log;
CREATE SCHEMA IF NOT EXISTS hist;
CREATE SCHEMA IF NOT EXISTS utils;
CREATE SCHEMA IF NOT EXISTS find;
CREATE SCHEMA IF NOT EXISTS tick;
CREATE SCHEMA IF NOT EXISTS agg;
--CREATE SCHEMA IF NOT EXISTS line;
/*
 
 
 타입 생성
 
 
 */
--CREATE TYPE ymxb_type AS enum('last_two_bound_up', 'last_two_bound_down');
CREATE TYPE price_type AS enum('CLOSE', 'OPEN', 'LOW', 'HIGH');
CREATE TYPE exchange_type AS enum('KOSPI', 'KOSDAQ', 'KOSDAQ_GLOBAL', 'KONEX');
CREATE TYPE country_type AS enum('KR', 'US');
-- 관측기간
CREATE TYPE obs_epoch AS enum(
    'WEEK',
    'MONTH',
    'QUARTER'
);
CREATE TYPE obs_target AS enum(
    'MIN',
    'MAX',
    'AVG'
);
CREATE TYPE agg_bound_type AS enum(
    --'CNT',  -- 반등 횟수= 로우수,  월별 작은값과 큰값 중간값이 안 나온다
    'CHG',  -- 가격 변화율
    'DAY'   -- x_cnt
);
-- 그래프방향 종류
CREATE TYPE direction_type AS enum(
    'DECREASE',
    'CONSTANT',
    'INCREASE'
);
-- bound_custom_line_type 종류 
-- k2: P1: 저가기존 뒤에서2번째 반등, P2: 1번째 반등
-- k3: P1: 고가기존 뒤에서2번째 반등, P2: 1번째 반등
CREATE TYPE bound_custom_line_type AS enum('K2', 'K3');
--  Periodtype 기간 종류
CREATE TYPE period_type AS enum('WEEKS', 'MONTH', 'QUARTER');
--  LOG_TASK_TAG 종류
CREATE TYPE log_task_tag AS enum(
    'TASK_EXE',
    'TASK_INPUT_DATA',
    'TASK_GRAPH_BOUND',
    'TASK_GRAPH_BOUND_YMXB',
    'TASK_AGG_VOLUME'
);
--  LOG_TASK_type 종류
CREATE TYPE log_task_type AS enum('INFO', 'ERROR');