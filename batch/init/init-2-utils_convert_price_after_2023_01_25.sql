/*
 
 ==========================================================================================================
 ==========================================================================================================
 utils - convert_price 2023년 1월 25일 개정 전이후 tick-size
 코스피 코스닥 코넥스 동일  호가단위 (2023년 1월 25일 시행) 
 2,000원 미만: 1원
 2,000원 이상 ~ 5,000원 미만: 5원
 5,000원 이상 ~ 20,000원 미만: 10원
 20,000원 이상 ~ 50,000원 미만: 50원
 50,000원 이상 ~ 200,000원 미만: 100원
 200,000원 이상 ~ 500,000원 미만: 500원
 500,000원 이상: 1,000원
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- convert_price 테이블
CREATE TABLE IF NOT EXISTS UTILS.Y_TICK (
	POINT NUMERIC(10, 0) NOT NULL,
	PRICE NUMERIC(10, 0) NOT NULL,
	UNIQUE (PRICE)
);

CREATE INDEX IF NOT EXISTS PRICE_IDX ON UTILS.Y_TICK (PRICE);

CREATE
OR REPLACE FUNCTION UTILS.MAKE_Y_TICK () RETURNS VOID AS $$
DECLARE is_point integer := -1;
tick integer := 0;
i INTEGER := 0;
-- 1주당 최대 가격 9백만원
exit_val INTEGER := 9999999;
BEGIN i := 0;
LOOP is_point := -1;
EXIT
WHEN i = exit_val;
IF i < 2000 THEN is_point := 1;
elsif i >= 2000 and i < 5000 then is_point := 1;
elsif i >= 5000 and i < 20000 and i % 5 = 0 then is_point := 1;
elsif i >= 20000 and i < 50000 and i % 50 = 0 then is_point := 1;
elsif i >= 50000 and i < 200000 and i % 100 = 0 then is_point := 1;
elsif i >= 200000 and i < 500000 and i % 500 = 0 then is_point := 1;
elsif i >= 500000 and i  %1000 = 0 then is_point := 1;
END IF;
IF is_point > -1 THEN
insert into utils.y_tick (point, price)
VALUES (tick, i);
SELECT tick + 1 INTO tick;
END IF;
SELECT i + 1 INTO i;
END LOOP;
END;
$$ LANGUAGE PLPGSQL;

SELECT
	*
FROM
	UTILS.MAKE_Y_TICK ();