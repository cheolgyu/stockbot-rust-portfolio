--drop view PUBLIC.view_rate_by_date;
CREATE OR REPLACE VIEW PUBLIC.view_rate_by_date AS
SELECT
	C.ID AS CODE_ID,
	C.STD_CODE,
	C.CODE,
	C.NAME,
	C.COUNTRY,
	C.EXCHANGE,
	v.dt,
	v.vol_rate,
	v.t_val_rate
FROM
	CODE C
	LEFT JOIN hist.rate_by_date V ON C.ID = V.CODE_ID
ORDER BY
	V.dt DESC,
	V.vol_rate DESC

-- drop table hist.rate_by_date;
CREATE TABLE IF NOT EXISTS hist.rate_by_date (
    code_id bigint NOT NULL,
    dt DATE NOT NULL,
    vol_rate numeric(6, 3)  NOT NULL,
    t_val_rate numeric(6, 3) NOT NULL,
    FOREIGN KEY (code_id) REFERENCES code (id),
    CONSTRAINT hist_rate_by_date_pk PRIMARY KEY(code_id, dt)
);
CREATE INDEX IF NOT EXISTS hist_rate_by_date_code_id_desc_idx ON hist.rate_by_date (code_id desc);
CREATE INDEX IF NOT EXISTS hist_rate_by_date_code_id_asc_idx ON hist.rate_by_date (code_id asc);
CREATE INDEX IF NOT EXISTS hist_rate_by_date_dt_desc_idx ON hist.rate_by_date (dt desc);
CREATE INDEX IF NOT EXISTS hist_rate_by_date_dt_asc_idx ON hist.rate_by_date (dt asc);


-- drop table hist.sum_by_date;
CREATE TABLE IF NOT EXISTS hist.sum_by_date (
    dt DATE NOT NULL,
    exchange exchange_type NOT NULL,
    sum_vol numeric NOT NULL,
    sum_t_val numeric NOT NULL,
    CONSTRAINT hist_sum_by_date_pk PRIMARY KEY( dt, exchange)
);
CREATE INDEX IF NOT EXISTS hist_sum_by_date_exchange_desc_idx ON hist.sum_by_date (exchange desc);
CREATE INDEX IF NOT EXISTS hist_sum_by_date_exchange_asc_idx ON hist.sum_by_date (exchange asc);
CREATE INDEX IF NOT EXISTS hist_sum_by_date_dt_desc_idx ON hist.sum_by_date (dt desc);
CREATE INDEX IF NOT EXISTS hist_sum_by_date_dt_asc_idx ON hist.sum_by_date (dt asc);

CREATE or replace FUNCTION hist.upsert_sum_by_date() RETURNS void AS $$
BEGIN 

INSERT INTO
	HIST.SUM_BY_DATE (DT, EXCHANGE, sum_vol, sum_t_val)
SELECT
	DT,
	C.EXCHANGE,
	SUM(VOL) AS sum_vol,
	SUM(t_val) AS sum_t_val
FROM
	CODE C
	LEFT JOIN HIST.PRICE P ON C.ID = P.CODE_ID
GROUP BY
	DT,
	EXCHANGE
ORDER BY
	DT ASC
ON CONFLICT (DT, EXCHANGE) DO UPDATE
SET
	sum_vol = EXCLUDED.sum_vol,
	sum_t_val = EXCLUDED.sum_t_val;

END;
$$ LANGUAGE plpgsql;
-- select hist.upsert_sum_by_date();

