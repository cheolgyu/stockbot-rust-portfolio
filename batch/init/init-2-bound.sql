/*
 
 ==========================================================================================================
 ==========================================================================================================
 hist - bound
 ==========================================================================================================
 ==========================================================================================================
 DROP TABLE HIST.BOUND CASCADE;
 DROP TABLE HIST.bound_daily CASCADE;
 */
--DROP TABLE HIST.BOUND CASCADE;
--DROP TABLE HIST.bound_daily CASCADE;
-- bound 테이블
CREATE TABLE IF NOT EXISTS hist.bound (
    code_id bigint NOT NULL,
    line price_type NOT NULL,
    p1x DATE NOT NULL,
    p1y numeric(7, 0) NOT NULL,
    p2x DATE NOT NULL,
    p2y numeric(7, 0) NOT NULL,
    change_rate numeric(10, 2) NOT NULL,
    direction direction_type NOT NULL,
    x_cnt integer NOT NULL,
    y_cnt numeric(7, 0) NOT NULL,
    --
    ymxb_p3y numeric(15, 2) NOT NULL,
    ymxb_m numeric(15, 2) NOT NULL,
    ymxb_b numeric(15, 2) NOT NULL,
    --
    PRIMARY KEY (code_id, line, p1x),
    FOREIGN KEY (code_id) REFERENCES code (id)
) PARTITION BY RANGE (p1x);
CREATE INDEX IF NOT EXISTS hist_bound_p1x_asc_idx ON hist.bound (p1x asc);
CREATE INDEX IF NOT EXISTS hist_bound_p1x_desc_idx ON hist.bound (p1x desc);
CREATE INDEX IF NOT EXISTS hist_bound_p2x_asc_idx ON hist.bound (p2x asc);
CREATE INDEX IF NOT EXISTS hist_bound_p2x_desc_idx ON hist.bound (p2x desc);
--CREATE INDEX test2_mm_idx ON test2 (major, minor);
DO $$
DECLARE query text;
BEGIN FOR counter IN 1952..2100 LOOP query := 'CREATE TABLE IF NOT EXISTS hist.bound_p1x_y' || counter || ' PARTITION OF hist.bound 
        FOR VALUES FROM (''' || counter || '-01-01'') TO (''' || counter || '-12-31''); ';
RAISE notice 'Value: %',
query;
EXECUTE query;
END LOOP;
END;
$$;
-- DROP TABLE IF  EXISTS hist.bound_view_tmp;
CREATE TABLE IF NOT EXISTS hist.bound_view_tmp (
    "code_id" bigint REFERENCES "code"(id),
    line price_type NOT NULL,
    "p1x" DATE,
    "p1y" numeric(7, 0),
    "p2x" DATE,
    "p2y" numeric(7, 0),
    "direction" direction_type NOT NULL,
    "x_cnt" integer,
    "y_cnt" numeric(7, 0),
    change_rate numeric(10, 2) NOT NULL,
    ymxb_p3y numeric(15, 2) NOT NULL,
    ymxb_m numeric(15, 2) NOT NULL,
    ymxb_b numeric(15, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT bound_view_tmp_idx PRIMARY KEY(code_id, line)
);
--DROP view PUBLIC.view_bound;
CREATE OR REPLACE VIEW PUBLIC.view_bound AS
SELECT C.ID,
    C.STD_CODE,
    C.CODE ,
    C.name,
    C.COUNTRY,
    C.EXCHANGE,
    bd.line,
    bd.CODE_ID,
    bd.P1X,
    bd.P1Y,
    bd.P2X,
    bd.P2Y,
    bd.DIRECTION,
    bd.X_CNT,
    bd.Y_CNT,
    bd.change_rate,
    bd.YMXB_P3Y,
    bd.YMXB_M,
    bd.YMXB_B,
    bd.CREATED_AT
FROM PUBLIC.code C
    LEFT JOIN hist.bound_view_tmp bd ON C.id = bd.code_id
where bd.code_id is not null
order by bd.change_rate  desc;