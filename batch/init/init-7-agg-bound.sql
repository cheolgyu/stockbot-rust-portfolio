
/*
 ==========================================================================================================
 ==========================================================================================================
 agg.bound
 ==========================================================================================================
 ==========================================================================================================
 */



    --drop table agg.bound_data CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.bound_data (
        code_id bigint NOT NULL,
        year numeric NOT NULL,
        v_epoch   numeric NOT NULL,
        obs_epoch obs_epoch NOT NULL,
        agg_bound_type agg_bound_type NOT NULL,
        v_target_avg numeric NOT NULL,
        v_target_min numeric NOT NULL,
        v_target_max numeric NOT NULL,
        rank_avg numeric NOT NULL,
        rank_min numeric NOT NULL,
        rank_max numeric NOT NULL,
        FOREIGN KEY (code_id) REFERENCES code (id),
        CONSTRAINT agg_bound_data_pk PRIMARY KEY(code_id, year, v_epoch, obs_epoch, agg_bound_type )
    );
    --drop table agg.bound CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.bound (
        code_id bigint NOT NULL,
        agg_bound_type agg_bound_type NOT NULL,
        obs_epoch obs_epoch NOT NULL,
        obs_target obs_target NOT NULL,
        v_epoch   numeric NOT NULL,
        v_epoch_cnt numeric NOT NULL,
        sum_v_epoch_cnt numeric NOT NULL,
        rate_all_epoch numeric NOT NULL,
        rate_epoch numeric NOT NULL,
        rank_num numeric NOT NULL,
        FOREIGN KEY (code_id) REFERENCES code (id),
        CONSTRAINT agg_bound_pk PRIMARY KEY(code_id, v_epoch , obs_epoch, obs_target, agg_bound_type)
    );
    
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    agg.insert_bound_data()
    bound_data: 한개의 연동에 대해 기간별 평균거래량,최소 거래량, 최대 거래량, 평균랭킹, 최소랭킹, 최대랭킹을 구함.

    Query returned successfully in 5 min 46 secs.

------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
    -- drop PROCEDURE agg.insert_bound_data();
    CREATE PROCEDURE agg.insert_bound_data() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.bound_data ;

    INSERT INTO agg.bound_data(code_id, year, v_epoch, obs_epoch, agg_bound_type, v_target_avg, v_target_min, v_target_max, rank_avg, rank_min, rank_max)
    select * from agg.select_agg_bound_data('WEEK','CHG','change_rate')
    UNION ALL
    select * from agg.select_agg_bound_data('WEEK','DAY','x_cnt')
    UNION ALL
    select * from agg.select_agg_bound_data('MONTH','CHG','change_rate')
    UNION ALL
    select * from agg.select_agg_bound_data('MONTH','DAY','x_cnt')
    UNION ALL
    select * from agg.select_agg_bound_data('QUARTER','CHG','change_rate')
    UNION ALL
    select * from agg.select_agg_bound_data('QUARTER','DAY','x_cnt')

;    
        
    COMMIT;
    ------------------------------`````````
    ------------------------------
    ------------------------------
    END;
    $$;
-- CALL agg.insert_bound_data();

   
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    agg.insert_bound()
    -- Query complete 00:00:04.776
    Query complete 00:02:00.662
    Query complete 00:00:05.681
    bound_data: 한개의 연도에 대해 기간별 평균거래량,최소 거래량, 최대 거래량, 평균랭킹, 최소랭킹, 최대랭킹을 구함.
    bound:      전체의 연도에 대한 전체 기간별 전체 타겟별      기간과 타겟의 비중과 랭킹 을구함 , 
    VIEW_bound 1:  거래량이 최고와 최소의 비중을 가진 기간을 구함.
    CALL

Query returned successfully in 1 min 8 secs.

------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
    -- drop PROCEDURE agg.insert_bound();
    CREATE PROCEDURE agg.insert_bound() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.bound ;
           
    INSERT INTO agg.bound(
        code_id
        ,obs_epoch
        ,v_epoch 
        ,obs_target
        ,agg_bound_type
        ,v_epoch_cnt
        ,sum_v_epoch_cnt
        ,rate_all_epoch
        ,rate_epoch
        ,RANK_NUM 
    )
    select * from agg.select_agg_bound('MIN','CHG','rank_min') UNION ALL 
    select * from agg.select_agg_bound('MIN','DAY','rank_min') UNION ALL 
    select * from agg.select_agg_bound('MAX','CHG','rank_MAX') UNION ALL 
    select * from agg.select_agg_bound('MAX','DAY','rank_MAX') UNION ALL 
    select * from agg.select_agg_bound('AVG','CHG','rank_AVG') UNION ALL 
    select * from agg.select_agg_bound('AVG','DAY','rank_AVG')  
    ;    
        
    COMMIT;
    ------------------------------`````````
    ------------------------------
    ------------------------------
    END;
    $$;
-- CALL agg.insert_bound();

/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
                             end CALL agg.insert_bound()
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/

/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_AGG_bound
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
--drop view PUBLIC.view_agg_bound_freq;
 CREATE OR REPLACE VIEW PUBLIC.view_agg_bound_freq AS
 SELECT
 	C.ID AS CODE_ID,
 	C.STD_CODE,
 	C.CODE,
 	C.NAME,
 	C.COUNTRY,
 	C.EXCHANGE,
 	V.obs_epoch,
 	V.obs_target,
    v.agg_bound_type,
 	V.v_epoch_cnt,
 	V.sum_v_epoch_cnt,
 	V.rate_all_epoch,
 	V.rate_epoch,
 	V.v_obs_agg
 FROM
 	CODE C
 	LEFT JOIN  (
 		select 
        	code_id, obs_epoch, agg_bound_type, obs_target, v_epoch_cnt, sum_v_epoch_cnt, rate_all_epoch, rate_epoch, array_agg(v_epoch order by v_epoch asc ) as v_obs_agg
        from agg.bound where rank_num = 1
        group by code_id, obs_epoch, agg_bound_type, obs_target, v_epoch_cnt, sum_v_epoch_cnt, rate_all_epoch, rate_epoch
     )v  ON C.ID = V.CODE_ID
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_AGG_bound_BEST END
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
