-- DROP TRIGGER insert_hist_price_point ON hist.price;
CREATE OR REPLACE FUNCTION insert_hist_price_point() RETURNS TRIGGER AS $insert_hist_price_point$
DECLARE var_exchange exchange_type;
var_date_point_id bigint;
BEGIN IF (TG_OP = 'INSERT') THEN
SELECT exchange INTO var_exchange
FROM code
WHERE id = NEW.code_id;
INSERT INTO utils.convert_date(exchange, dt)
VALUES (var_exchange, NEW.dt) ON CONFLICT (exchange, dt) DO NOTHING;
SELECT id INTO var_date_point_id
FROM utils.convert_date
WHERE exchange = var_exchange
    AND dt = NEW.dt;
NEW.dt_point_id = var_date_point_id;
NEW.op_point = (
    SELECT point
    FROM utils.convert_price
    WHERE price = NEW.op
        AND exchange = var_exchange
);
NEW.cp_point = (
    SELECT point
    FROM utils.convert_price
    WHERE price = NEW.cp
        AND exchange = var_exchange
);
NEW.lp_point = (
    SELECT point
    FROM utils.convert_price
    WHERE price = NEW.lp
        AND exchange = var_exchange
);
NEW.hp_point = (
    SELECT point
    FROM utils.convert_price
    WHERE price = NEW.hp
        AND exchange = var_exchange
);
-- elsif (TG_OP = 'UPDATE') THEN
--     SELECT exchange INTO var_exchange FROM code WHERE id = NEW.code_id ; 
--     INSERT INTO utils.convert_date( exchange,  dt) VALUES ( var_exchange , NEW.dt ) ON CONFLICT (exchange ,dt) DO NOTHING;
--     SELECT id INTO var_date_point_id FROM utils.convert_date WHERE exchange = var_exchange AND dt = NEW.dt; 
--     NEW.dt_point_id = var_date_point_id;
--     NEW.op_point = (SELECT point FROM utils.convert_price WHERE price = NEW.op AND exchange = var_exchange) ;
--     NEW.cp_point = (SELECT point FROM utils.convert_price WHERE price = NEW.cp AND exchange = var_exchange) ;
--     NEW.lp_point = (SELECT point FROM utils.convert_price WHERE price = NEW.lp AND exchange = var_exchange) ;
--     NEW.hp_point = (SELECT point FROM utils.convert_price WHERE price = NEW.hp AND exchange = var_exchange) ;
END IF;
RETURN NEW;
END;
$insert_hist_price_point$ LANGUAGE plpgsql;
CREATE TRIGGER insert_hist_price_point BEFORE
INSERT
    OR
UPDATE ON hist.price FOR EACH ROW EXECUTE FUNCTION insert_hist_price_point();
------------------------------
------------------------------
------------------------------
------------------------------