/*
 
 ==========================================================================================================
 ==========================================================================================================
 find
 ==========================================================================================================
 ==========================================================================================================
 
 */
/*
 B00    'bound',
 B10    'bound_today_same',
 B11    'bound_today_low_up',
 B12    'bound_today_low_downing',
 V00    'volume'
 
 
 F   'foot',
 K   'knee',
 S   'shoulder',
 H   'head',
 */
CREATE TYPE find_why AS enum(
    'B00',
    'B10',
    'B11',
    'B12',
    'V00'
);
-- ALTER TYPE find_why RENAME VALUE 'b0', TO 'B0',;
-- ALTER TYPE find_why RENAME VALUE 'b10' TO 'B10';
-- ALTER TYPE find_why RENAME VALUE 'b11' TO 'B11';
-- ALTER TYPE find_why RENAME VALUE 'b12' TO 'B12';
-- -- ALTER TYPE find_why RENAME VALUE 'v0' TO 'V0';
-- ALTER TYPE find_why RENAME VALUE 'B0' TO 'B00';
-- ALTER TYPE find_why RENAME VALUE 'V0' TO 'V00';
CREATE TYPE find_what AS enum('F', 'K', 'S', 'H');
-- ALTER TYPE find_what RENAME VALUE 'f' TO 'F';
-- ALTER TYPE find_what RENAME VALUE 'k' TO 'K';
-- ALTER TYPE find_what RENAME VALUE 's' TO 'S';
-- ALTER TYPE find_what RENAME VALUE 'h' TO 'H';
-- drop table hist.find;
CREATE TABLE IF NOT EXISTS hist.find (
    dt DATE NOT NULL,
    what find_what NOT NULL,
    why find_why NOT NULL,
    ORDER_NUM integer NOT NULL,
    --  
    --  hist.bound UNIQUE (code_id, line, p1x)
    code_id bigint NOT NULL,
    line price_type NOT NULL,
    p1x DATE NOT NULL,
    --
    -- 
    change_rate numeric NOT NULL,
    UNIQUE (dt, what, why, ORDER_NUM),
    FOREIGN KEY (code_id) REFERENCES CODE (id)
) PARTITION BY RANGE (dt);
CREATE INDEX IF NOT EXISTS dt_idx ON hist.find(dt);
DO $$
DECLARE query text;
BEGIN FOR counter IN 2020..2100 LOOP query := 'CREATE TABLE IF NOT EXISTS hist.find_dt' || counter || ' PARTITION OF hist.find 
        FOR VALUES FROM (''' || counter || '-01-01'') TO (''' || counter || '-12-31''); ';
RAISE notice 'Value: %',
query;
EXECUTE query;
END LOOP;
END;
$$;
--------------------------------------------------------------------
--------------------------------------------------------------------
--------------------------------------------------------------------
--------------------------------------------------------------------
--------------------------------------------------------------------
------> 오류담  UTILS.CONVERT_DATE 변경바람.
CREATE OR REPLACE PROCEDURE hist.insert_find() AS $$
declare r RECORD;
BEGIN --------------------------------------------------------------------
FOR r IN WITH TA AS (
    SELECT DT,
        POINT
    FROM UTILS.CONVERT_DATE
    WHERE EXCHANGE = 'KOSPI'
    ORDER BY ID
),
TB AS (
    SELECT TN.DT AS TODAY,
        TA.DT AS TODAY_MIN1
    FROM UTILS.CONVERT_DATE AS TN
        LEFT JOIN TA ON TN.POINT = TA.POINT + 1
    WHERE TN.EXCHANGE = 'KOSPI'
    ORDER BY TN.ID
),
TC AS(
    SELECT TB.TODAY,
        TB.TODAY_MIN1,
        COUNT(
            CASE
                WHEN F.WHAT = 'F' THEN 1
            END
        ) AS F,
        COUNT(
            CASE
                WHEN F.WHAT = 'K' THEN 1
            END
        ) AS K,
        COUNT(
            CASE
                WHEN F.WHAT = 'S' THEN 1
            END
        ) AS S,
        COUNT(
            CASE
                WHEN F.WHAT = 'H' THEN 1
            END
        ) AS H
    FROM TB
        LEFT JOIN HIST.FIND F ON TB.TODAY = F.DT
    GROUP BY TB.TODAY,
        TB.TODAY_MIN1
    ORDER BY TB.TODAY ASC
)
SELECT *
FROM TC
WHERE f = 0
    or k = 0
    or s = 0
    or h = 0 LOOP IF r.f = 0 THEN PERFORM HIST.INSERT_FIND_FOOT(r.today);
END IF;
IF r.k = 0 THEN PERFORM HIST.INSERT_FIND_KNEE(r.today, r.today_min1);
END IF;
IF r.s = 0 THEN PERFORM HIST.INSERT_FIND_SHOULDER(r.today, r.today_min1);
END IF;
IF r.h = 0 THEN PERFORM HIST.INSERT_FIND_HEAD(r.today);
END IF;
END LOOP;
--------------------------------------------------------------------
end;
$$LANGUAGE PLPGSQL;
--CALL hist.insert_find();
CALL hist.insert_find();
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------------------------------------------------------
/*
 
 ---------------------------------------------------------------------
 
 
 
 
 ---------------------------------------------------------------------
 
 */
CREATE OR REPLACE FUNCTION HIST.INSERT_FIND_FOOT(INPUT_DT date) RETURNS VOID AS $$
DECLARE OPEN_DT_1_DAY_AGO date;
BEGIN
INSERT INTO hist.find
SELECT P2X AS dt,
    'F'::find_what as what,
    'B00'::find_why as why,
    ROW_NUMBER() OVER(
        ORDER BY change_rate DESC
    ) AS ORDER_NUM,
    CODE_ID,
    LINE,
    P1X,
    change_rate
FROM HIST.BOUND
WHERE LINE = 'CLOSE'
    AND P2X = INPUT_DT
    AND DIRECTION = 'DECREASE'
ORDER BY X_CNT,
    change_rate DESC;
END;
$$ LANGUAGE PLPGSQL;
/*
 
 ---------------------------------------------------------------------
 
 
 
 
 ---------------------------------------------------------------------
 
 */
CREATE OR REPLACE FUNCTION HIST.INSERT_FIND_KNEE(INPUT_DT date, OPEN_DT_1_DAY_AGO date) RETURNS VOID AS $$ BEGIN
INSERT INTO hist.find with t_foot_day_min_1 as (
        select t.*
        from hist.find t
        where t.dt = OPEN_DT_1_DAY_AGO
            AND t.what = 'F'::find_what
        order by t.ORDER_NUM desc
        limit 100
    ) ---
, find_t1 as (
        select tb.P2X AS dt,
            'K'::find_what as what,
            'B10'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_foot_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'CONSTANT'
            ) tb on t1.code_id = tb.code_id
        order by tb.change_rate DESC
    ),
    find_t2 as (
        select tb.P2X AS dt,
            'K'::find_what as what,
            'B11'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_foot_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'INCREASE'
                order by change_rate asc
            ) tb on t1.code_id = tb.code_id
        order by tb.change_rate DESC
    ),
    find_t3 as (
        select tb.P2X AS dt,
            'K'::find_what as what,
            'B12'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_foot_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'DECREASE'
                order by change_rate asc
            ) tb on t1.code_id = tb.code_id
            AND t1.change_rate >= tb.change_rate
        order by tb.change_rate DESC
    )
SELECT *
from find_t1
union all
SELECT *
from find_t2
union all
SELECT *
from find_t3;
END;
$$ LANGUAGE PLPGSQL;
/*
 
 ---------------------------------------------------------------------
 
 
 
 
 ---------------------------------------------------------------------
 
 */
CREATE OR REPLACE FUNCTION HIST.INSERT_FIND_SHOULDER(INPUT_DT date, OPEN_DT_1_DAY_AGO date) RETURNS VOID AS $$ BEGIN
INSERT INTO hist.find with t_head_day_min_1 as (
        select t.*
        from hist.find t
        where t.dt = OPEN_DT_1_DAY_AGO
            AND t.what = 'H'::find_what
        order by t.ORDER_NUM desc
        limit 100
    ) ---
, find_t1 as (
        select tb.P2X AS dt,
            'S'::find_what as what,
            'B10'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_head_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'CONSTANT'
            ) tb on t1.code_id = tb.code_id
        order by tb.change_rate DESC
    ),
    find_t2 as (
        select tb.P2X AS dt,
            'S'::find_what as what,
            'B11'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_head_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'INCREASE'
                order by change_rate asc
            ) tb on t1.code_id = tb.code_id
        order by tb.change_rate DESC
    ),
    find_t3 as (
        select tb.P2X AS dt,
            'S'::find_what as what,
            'B12'::find_why as why,
            ROW_NUMBER() OVER(
                ORDER BY tb.change_rate DESC
            ) AS ORDER_NUM,
            tb.CODE_ID,
            tb.LINE,
            tb.P1X,
            tb.change_rate
        from t_head_day_min_1 t1
            inner join (
                select *
                from hist.bound
                WHERE LINE = 'CLOSE'
                    AND P2X = INPUT_DT
                    AND DIRECTION = 'DECREASE'
                order by change_rate asc
            ) tb on t1.code_id = tb.code_id
            AND t1.change_rate >= tb.change_rate
        order by tb.change_rate DESC
    )
SELECT *
from find_t1
union all
SELECT *
from find_t2
union all
SELECT *
from find_t3;
END;
$$ LANGUAGE PLPGSQL;
/*
 
 ---------------------------------------------------------------------
 
 
 
 
 ---------------------------------------------------------------------
 
 */
CREATE OR REPLACE FUNCTION HIST.INSERT_FIND_HEAD(INPUT_DT date) RETURNS VOID AS $$ BEGIN
INSERT INTO hist.find
SELECT P2X AS dt,
    'H'::find_what as what,
    'B00'::find_why as why,
    ROW_NUMBER() OVER(
        ORDER BY change_rate DESC
    ) AS ORDER_NUM,
    CODE_ID,
    LINE,
    P1X,
    change_rate
FROM HIST.BOUND
WHERE LINE = 'CLOSE'
    AND P2X = INPUT_DT
    AND DIRECTION = 'INCREASE'
ORDER BY X_CNT,
    change_rate DESC;
END;
$$ LANGUAGE PLPGSQL;
-- select * from hist.insert_find();
---------------------------------------------------------------------
CREATE OR REPLACE VIEW PUBLIC.VIEW_FIND AS
SELECT C.ID,
    CD.SHORT_CODE,
    CD.SHORT_NAME,
    C.EXCHANGE,
    C.COUNTRY,
    T.DT,
    T.WHAT,
    T.WHY,
    T.ORDER_NUM,
    T.LINE,
    T.P1X,
    T.change_rate
FROM (
        SELECT MAX(DT)
        FROM UTILS.CONVERT_DATE
        WHERE EXCHANGE = 'KOSPI'
    ) D
    LEFT JOIN HIST.FIND T ON D.MAX = T.DT
    LEFT JOIN CODE C ON T.CODE_ID = C.ID
    LEFT JOIN CODE_DETAIL CD ON C.ID = CD.CODE_ID;