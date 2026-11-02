/*
 
 ==========================================================================================================
 ==========================================================================================================
 tick.X_TICK 
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- convert_date 테이블
-- DROP TABLE utils.convert_date CASCADE; 
CREATE TABLE IF NOT EXISTS tick.X_TICK (
    exchange exchange_type NOT NULL,
    dt DATE NOT NULL,
    tick bigint not NULL,
    UNIQUE (exchange, dt)
);
CREATE INDEX IF NOT EXISTS x_tick_idx ON tick.X_TICK (exchange, dt);
CREATE INDEX IF NOT EXISTS x_tick_dt_asc_idx ON tick.X_TICK (dt asc);
CREATE INDEX IF NOT EXISTS x_tick_dt_desc_idx ON tick.X_TICK (dt desc);
CREATE PROCEDURE tick.upsert_x_tick() LANGUAGE plpgsql AS $$ BEGIN ------------------------------
------------------------------
------------------------------
INSERT INTO TICK.X_TICK (EXCHANGE, DT, TICK)
SELECT C.EXCHANGE,
    P.DT,
    RANK() OVER (
        PARTITION BY C.EXCHANGE
        ORDER BY P.DT ASC
    ) TICK
FROM HIST.PRICE P
    LEFT JOIN CODE C ON P.CODE_ID = C.ID
GROUP BY C.EXCHANGE,
    P.DT ON CONFLICT (EXCHANGE, DT) DO
UPDATE
SET TICK = EXCLUDED.TICK;
COMMIT;
------------------------------
------------------------------
------------------------------
END;
$$;
/*
 
 ==========================================================================================================
 ==========================================================================================================
 tick.Y_TICK 
 ==========================================================================================================
 ==========================================================================================================
 
 */
CREATE TABLE IF NOT EXISTS tick.y_TICK (
    exchange exchange_type NOT NULL,
    dt DATE NOT NULL,
    tick bigint not NULL,
    UNIQUE (exchange, dt)
);
CREATE INDEX IF NOT EXISTS x_tick_idx ON tick.y_TICK (exchange, dt);