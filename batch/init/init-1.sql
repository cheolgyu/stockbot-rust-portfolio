/*
 
 
 테이블 생성
 
 
 */
-- 코드 테이블
CREATE TABLE IF NOT EXISTS CODE (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR UNIQUE,
    code VARCHAR UNIQUE,
    std_code VARCHAR UNIQUE,
    country country_type NOT NULL,
    exchange exchange_type NOT NULL
);
CREATE INDEX code_code_index ON code(code);
CREATE INDEX code_name_index ON code(name);
-- 코드 상세 테이블
CREATE TABLE IF NOT EXISTS CODE_DETAIL (
    code_id BIGSERIAL PRIMARY KEY,
    std_code VARCHAR UNIQUE,
    short_code VARCHAR,
    std_name VARCHAR,
    short_name VARCHAR,
    eng_name VARCHAR,
    listing_date VARCHAR,
    exchange exchange_type,
    securities_type VARCHAR,
    company_type VARCHAR,
    stock_type VARCHAR,
    face_value VARCHAR,
    stock_cnt VARCHAR,
    FOREIGN KEY (code_id) REFERENCES CODE (id)
);
-- 코드 상태 테이블
CREATE TABLE IF NOT EXISTS CODE_STATE (
    code_id BIGSERIAL PRIMARY KEY,
    short_code VARCHAR UNIQUE,
    -- 매매거래정지
    stop bool,
    -- 정리매매 종목	
    clear bool,
    --관리종목	
    manger bool,
    --투자주의환기종목
    alert bool,
    --불성실공시	
    unfaithful bool,
    --단일가매매대상 초저유동성종목	
    low_liquidity bool,
    --상장주식수 부족 우선주
    shortage bool,
    --단기과열종목
    overheated bool,
    --투자주의종목
    caution bool,
    --투자경고종목
    warning bool,
    --투자위험종목
    risk bool,
    FOREIGN KEY (code_id) REFERENCES CODE (id)
);
CREATE OR REPLACE VIEW PUBLIC.VIEW_CODE AS
SELECT C.ID,
    C.STD_CODE,
    C.COUNTRY,
    C.EXCHANGE,
    CD.CODE_ID,
    CD.SHORT_CODE,
    CD.STD_NAME,
    CD.SHORT_NAME,
    CD.ENG_NAME,
    CD.LISTING_DATE,
    CD.SECURITIES_TYPE,
    CD.COMPANY_TYPE,
    CD.STOCK_TYPE,
    CD.FACE_VALUE,
    CD.STOCK_CNT
FROM CODE C
    LEFT JOIN CODE_DETAIL CD ON C.ID = CD.CODE_ID;
/*
 
 ==========================================================================================================
 ==========================================================================================================
 hist - price
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- 가격 테이블
CREATE TABLE IF NOT EXISTS hist.price (
    code_id bigint NOT NULL,
    dt DATE NOT NULL,
    op numeric(7, 0) NOT NULL,
    hp numeric(7, 0) NOT NULL,
    lp numeric(7, 0) NOT NULL,
    cp numeric(7, 0) NOT NULL,
     -- 거래량
    vol bigint NOT NULL,
      -- 거래 대금
    t_val bigint NOT NULL,
    -- 상장 주식수
    stock_cnt bigint NOT NULL,
     -- 시가총액
    capital bigint NOT NULL,
    PRIMARY KEY (code_id, dt),
    FOREIGN KEY (code_id) REFERENCES code (id)
) PARTITION BY RANGE (dt);
------------------------------
------------------------------
------------------------------
------------------------------
CREATE INDEX IF NOT EXISTS price_dt_asc_idx ON hist.price (dt ASC);
CREATE INDEX IF NOT EXISTS price_dt_desc_idx ON hist.price (dt desc);
CREATE INDEX IF NOT EXISTS price_code_id_asc_idx ON hist.price (code_id ASC);
CREATE INDEX IF NOT EXISTS price_code_id_desc_idx ON hist.price (code_id desc);
CREATE INDEX IF NOT EXISTS price_code_id_dt_idx ON hist.price (code_id,dt);
DO $$
DECLARE query text;
BEGIN FOR counter IN 1995..2100 LOOP query := 'CREATE TABLE IF NOT EXISTS hist.price_y' || counter || ' PARTITION OF hist.price 
        FOR VALUES FROM (''' || counter || '-01-01'') TO (''' || counter || '-12-31''); ';
RAISE notice 'Value: %',
query;
EXECUTE query;
END LOOP;
END;
$$;
------------------------------
------------------------------
------------------------------
------------------------------