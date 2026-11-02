/*
 
 ==========================================================================================================
 ==========================================================================================================
 utils - convert_date
 ==========================================================================================================
 ==========================================================================================================
 
 */
-- convert_date 테이블
-- DROP TABLE utils.convert_date CASCADE; 
CREATE TABLE IF NOT EXISTS utils.X_TICK (
    id BIGSERIAL PRIMARY KEY,
    exchange exchange_type NOT NULL,
    point numeric(9, 0) NULL,
    dt DATE NOT NULL,
    UNIQUE (exchange, dt)
);
CREATE INDEX IF NOT EXISTS dt_idx ON utils.X_TICK (exchange, dt);
-- convert_date 테이블 update point 함수.
CREATE or replace FUNCTION utils.update_x_tick() RETURNS void AS $$
BEGIN FOR var_exchange in
select unnest(enum_range(NULL::exchange_type)) loop
UPDATE utils.X_TICK xt
set point = b.point
from (
        select id,
            exchange,
            dt,
            RANK () OVER (
                ORDER BY dt ASC
            ) point
        from utils.X_TICK
        where exchange = var_exchange
        order by dt desc
    ) b
where xt.exchange = var_exchange
    AND xt.id = b.id;
end loop;
END;
$$ LANGUAGE plpgsql;
-- select * from utils.update_x_tick();