/*
===============================================
저가 에서 고가 뺴기.  + 부호는 시가가 종가보다 높으면 양수 반대는 음수
tb_name= hist.price_chg
===============================================

*/


drop table hist.price_chg;
CREATE TABLE IF NOT EXISTS hist.price_chg (
    code_id bigint NOT NULL,
    dt DATE NOT NULL,
    -- 저가 고가 변화율
    lh numeric(10, 2) NOT NULL,
    -- 시가 종가 변화율
    oc numeric(10, 2) NOT NULL,
    -- 전날과의 거래량 변화율 
    vol numeric(20, 2) NOT NULL,
    PRIMARY KEY (code_id, dt),
    FOREIGN KEY (code_id) REFERENCES code (id),
    FOREIGN KEY (code_id, dt) REFERENCES hist.price (code_id, dt)
);
/*
===============================================
CALL

Query returned successfully in 12 min 16 secs.
===============================================

*/
 drop PROCEDURE hist.insert_price_chg();
    CREATE PROCEDURE hist.insert_price_chg() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
           
    INSERT INTO hist.price_chg(
        code_id
        ,dt
        ,lh 
        ,oc
        ,vol
    )
    select 
        p.code_id, p.dt
        ,case when lp = 0  then 0
        else round(((hp-lp)::NUMERIC /lp)*100,2 ) 
        end as chg_lh
        ,case when op = 0  then 0
        else round(((cp-op)::NUMERIC /op)*100,2 ) 
        end as chg_oc
        ,
        case when LAG( p.vol ) over ( partition by p.code_id  order by p.dt asc) is null then 0
        when LAG( p.vol ) over ( partition by p.code_id  order by p.dt asc) = 0 then 0
        else 
            round((
                (p.vol - LAG( p.vol ) over ( partition by p.code_id  order by p.dt asc)  )::NUMERIC 
                    /
                LAG( p.vol ) over ( partition by p.code_id  order by p.dt asc)
                )*100,2 ) 
        end
       
         as chg_vol

    from hist.price p 
        left join hist.price_chg pc on  p.code_id = pc.code_id and p.dt=pc.dt
    where pc.code_id is null	
    ;    
        
    COMMIT;
    ------------------------------`````````
    ------------------------------
    ------------------------------
    END;
    $$;
 call hist.insert_price_chg();    
 /*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_price_chg
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
--drop view PUBLIC.view_price_chg;
 CREATE OR REPLACE VIEW PUBLIC.view_price_chg AS
 SELECT
 	C.ID AS CODE_ID,
 	C.STD_CODE,
 	C.CODE,
 	C.NAME,
 	C.COUNTRY,
 	C.EXCHANGE,
    v.dt,
 	V.lh,
 	V.oc,
    v.vol
 FROM
 	CODE C
 	LEFT JOIN  hist.price_chg v  ON C.ID = V.CODE_ID
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_price_chg  END
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/