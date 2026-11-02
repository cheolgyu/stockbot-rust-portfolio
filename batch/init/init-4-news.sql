/*
 
 ==========================================================================================================
 ==========================================================================================================
 hist - news
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- bound 테이블
CREATE TABLE IF NOT EXISTS hist.news (
    code_id bigint NOT NULL,
    dt DATE NOT NULL,
    title TEXT NOT NULL,
    link varchar NOT NULL,
    media_type varchar NOT NULL,
    UNIQUE (code_id, dt, link),
    FOREIGN KEY (code_id) REFERENCES code (id)
) PARTITION BY RANGE (dt);
CREATE INDEX IF NOT EXISTS p1x_dt ON hist.news (dt);
--CREATE INDEX test2_mm_idx ON test2 (major, minor);
DO $$
DECLARE query text;
BEGIN FOR counter IN 2020..2100 LOOP query := 'CREATE TABLE IF NOT EXISTS hist.news_dt' || counter || ' PARTITION OF hist.news 
        FOR VALUES FROM (''' || counter || '-01-01'') TO (''' || counter || '-12-31''); ';
RAISE notice 'Value: %',
query;
EXECUTE query;
END LOOP;
END;
$$;