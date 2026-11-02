/*
 
 ==========================================================================================================
 ==========================================================================================================
 utils - convert_price 2023년 1월 25일 개정 전
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- convert_price 테이블
CREATE TABLE IF NOT EXISTS utils.convert_price (
    exchange exchange_type NOT NULL,
    point numeric NOT NULL,
    price numeric NOT NULL,
    UNIQUE (exchange, price)
);
CREATE INDEX IF NOT EXISTS price_idx ON utils.convert_price (exchange, price);
CREATE or replace FUNCTION utils.insert_convert_price_kospi() RETURNS void AS $$
DECLARE ok integer := -1;
tick integer := 0;
i INTEGER := 0;
-- 5백만원
exit_val INTEGER := 5999999;
BEGIN i := 0;
LOOP ok := -1;
EXIT
WHEN i = exit_val;
IF i < 1000 THEN ok := 1;
elsif i >= 1000
and i < 5000 then ok := 1;
elsif i >= 5000
and i < 10000
and i %5 = 0 then ok := 1;
elsif i >= 10000
and i < 50000
and i %50 = 0 then ok := 1;
elsif i >= 50000
and i < 100000
and i %100 = 0 then ok := 1;
elsif i >= 100000
and i < 500000
and i %500 = 0 then ok := 1;
elsif i >= 500000
and i %1000 = 0 then ok := 1;
END IF;
IF ok > -1 THEN
insert into utils.convert_price (exchange, point, price)
VALUES ('KOSPI', tick, i);
SELECT tick + 1 INTO tick;
END IF;
SELECT i + 1 INTO i;
END LOOP;
END;
$$ LANGUAGE plpgsql;
select *
from utils.insert_convert_price_kospi();
CREATE or replace FUNCTION utils.insert_convert_price_not_kospi() RETURNS void AS $$
DECLARE ok integer := -1;
i INTEGER := 0;
tick integer := 0;
exit_val INTEGER := 5999999;
BEGIN i := 0;
LOOP ok := -1;
EXIT
WHEN i = exit_val;
IF i < 1000 THEN ok := 1;
elsif i >= 1000
and i < 5000
and i %5 = 0 then ok := 1;
elsif i >= 5000
and i < 10000
and i %10 = 0 then ok := 1;
elsif i >= 10000
and i < 50000
and i %50 = 0 then ok := 1;
elsif i >= 50000
and i < 100000
and i %100 = 0 then ok := 1;
-- elsif i >= 100000 and  i < 500000 and ( market = 1 and i%500 = 0 ) then
--     ok := 1; 
elsif i >= 100000
and i < 500000
and i %100 = 0 then ok := 1;
-- elsif i >= 500000 and  (market = 1 and i%1000 = 0 )  then
--     ok := 1;  
elsif i >= 500000
and i %100 = 0 then ok := 1;
END IF;
IF ok > -1 THEN
insert into utils.convert_price (exchange, point, price)
VALUES ('KOSDAQ', tick, i);
insert into utils.convert_price (exchange, point, price)
VALUES ('KONEX', tick, i);
SELECT tick + 1 INTO tick;
END IF;
SELECT i + 1 INTO i;
END LOOP;
END;
$$ LANGUAGE plpgsql;
select *
from utils.insert_convert_price_not_kospi();