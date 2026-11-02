
/*
 ==========================================================================================================
 ==========================================================================================================
 agg.vol
 ==========================================================================================================
 ==========================================================================================================
 */



    --drop table agg.vol_data CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.vol_data (
        code_id bigint NOT NULL,
        year numeric NOT NULL,
        obs_epoch obs_epoch NOT NULL,
        v_epoch   numeric NOT NULL,
        v_target_avg numeric NOT NULL,
        v_target_min numeric NOT NULL,
        v_target_max numeric NOT NULL,
        rank_avg numeric NOT NULL,
        rank_min numeric NOT NULL,
        rank_max numeric NOT NULL,
        FOREIGN KEY (code_id) REFERENCES code (id),
        CONSTRAINT agg_vol_data_pk PRIMARY KEY(code_id, year, obs_epoch, v_epoch )
    );
    --drop table agg.vol CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.vol (
        code_id bigint NOT NULL,
        obs_epoch obs_epoch NOT NULL,
        obs_target obs_target NOT NULL,
        v_epoch   numeric NOT NULL,
        v_epoch_cnt numeric NOT NULL,
        sum_v_epoch_cnt numeric NOT NULL,
        rate_all_epoch numeric NOT NULL,
        rate_epoch numeric NOT NULL,
        rank_num numeric NOT NULL,
        FOREIGN KEY (code_id) REFERENCES code (id),
        CONSTRAINT agg_vol_pk PRIMARY KEY(code_id, obs_epoch, v_epoch, obs_target)
    );
    
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    agg.insert_vol_data()
    -- Query complete 00:02:00.738
    Query complete 00:02:00.662
    vol_data: 한개의 연동에 대해 기간별 평균거래량,최소 거래량, 최대 거래량, 평균랭킹, 최소랭킹, 최대랭킹을 구함.
    
    Query complete 00:00:19.183 insert 제외
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
    -- drop PROCEDURE agg.insert_vol_data();
    CREATE PROCEDURE agg.insert_vol_data() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.vol_data ;

    INSERT INTO agg.vol_data(code_id, year, obs_epoch, v_epoch, v_target_avg, v_target_min, v_target_max, rank_avg, rank_min, rank_max)
    select
        code_id, year, obs_epoch, v_epoch, v_target_avg, v_target_min::numeric, v_target_max::numeric
        ,rank() OVER (PARTITION BY code_id, year, obs_epoch  ORDER BY v_target_avg DESC)::numeric as rank_avg
        ,rank() OVER (PARTITION BY code_id, year, obs_epoch  ORDER BY v_target_min ASC)::numeric as rank_min
        ,rank() OVER (PARTITION BY code_id, year, obs_epoch  ORDER BY v_target_max DESC)::numeric as rank_max
	from ( 
        select 
            'MONTH'::obs_epoch AS obs_epoch,
            code_id,
            EXTRACT(year FROM dt) AS year,
            EXTRACT(month FROM dt) AS v_epoch,
            TRUNC(avg(vol),2) as v_target_avg, 
            min(vol)as v_target_min, 
            max(vol) as v_target_max 
        from hist.price
        GROUP BY code_id, year, v_epoch 
        UNION ALL
        select 
            'WEEK'::obs_epoch AS obs_epoch,
            code_id,
            EXTRACT(year FROM dt) AS year,
            EXTRACT(WEEK FROM dt) AS v_epoch,
            TRUNC(avg(vol),2) as v_target_avg, 
            min(vol)as v_target_min, 
            max(vol) as v_target_max 
        from hist.price
        GROUP BY code_id, year, v_epoch 
        UNION ALL
        select 
            'QUARTER'::obs_epoch AS obs_epoch,
            code_id,
            EXTRACT(year FROM dt) AS year,
            EXTRACT(QUARTER FROM dt) AS v_epoch,
            TRUNC(avg(vol),2) as v_target_avg, 
            min(vol)as v_target_min, 
            max(vol) as v_target_max 
        from hist.price
        GROUP BY code_id, year, v_epoch 	
	)

;    
        
    COMMIT;
    ------------------------------`````````
    ------------------------------
    ------------------------------
    END;
    $$;
-- CALL agg.insert_vol_data();

   
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    agg.insert_vol()
    -- Query complete 00:00:04.776
    Query complete 00:02:00.662
    Query complete 00:00:05.681
    vol_data: 한개의 연도에 대해 기간별 평균거래량,최소 거래량, 최대 거래량, 평균랭킹, 최소랭킹, 최대랭킹을 구함.
    vol:      전체의 연도에 대한 전체 기간별 전체 타겟별      기간과 타겟의 비중과 랭킹 을구함 , 
    VIEW_VOL 1:  거래량이 최고와 최소의 비중을 가진 기간을 구함.
    CALL

Query returned successfully in 32 secs 372 msec.
Query complete 00:00:36.027

------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
    -- drop PROCEDURE agg.insert_vol();
    CREATE PROCEDURE agg.insert_vol() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.vol ;
           
    INSERT INTO agg.vol(
        code_id
        ,obs_target
        ,obs_epoch
        ,v_epoch 
        ,v_epoch_cnt
        ,sum_v_epoch_cnt
        ,rate_all_epoch
        ,rate_epoch
        ,RANK_NUM 
    )
    with tb_sub as (
       select 
            'MAX'::obs_target AS obs_target
            ,obs_epoch
            ,code_id
            ,v_epoch 
			,coalesce(t2_v_epoch_cnt,0) as v_epoch_cnt
			,coalesce(sum_v_epoch_cnt,0) AS sum_v_epoch_cnt
            ,TRUNC( coalesce(t2_v_epoch_cnt,0) / sum(COUNT(*)) OVER ( PARTITION BY CODE_ID, obs_epoch  )  *100, 2)  as rate_all_epoch
			,coalesce(TRUNC( t2_v_epoch_cnt  / sum_v_epoch_cnt  *100, 2),0)  as rate_epoch
            
        from agg.vol_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
                , COUNT(*)  AS t2_v_epoch_cnt
				,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.vol_data  where rank_max =1
            group by code_id, obs_epoch, v_epoch ,rank_max  
        ) t2 on t1.code_id= t2_code_id and t1.obs_epoch= t2_obs_epoch and t1.v_epoch= t2_v_epoch
        group by code_id, obs_epoch, v_epoch ,t2_v_epoch_cnt,sum_v_epoch_cnt
        
        UNION ALL
        
       select 
            'MIN'::obs_target AS obs_target
            ,obs_epoch
            ,code_id
            ,v_epoch 
            ,coalesce(t2_v_epoch_cnt,0) as v_epoch_cnt
			,coalesce(sum_v_epoch_cnt,0) AS sum_v_epoch_cnt
            ,TRUNC( coalesce(t2_v_epoch_cnt,0) / sum(COUNT(*)) OVER ( PARTITION BY CODE_ID, obs_epoch  )  *100, 2)  as rate_all_epoch
			,coalesce(TRUNC( t2_v_epoch_cnt  / sum_v_epoch_cnt  *100, 2),0)  as rate_epoch
            
        from agg.vol_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
				,COUNT(*)  AS t2_v_epoch_cnt
                ,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.vol_data   where rank_min =1
            group by code_id, obs_epoch, v_epoch ,rank_min  
        ) t2 on t1.code_id= t2_code_id and t1.obs_epoch= t2_obs_epoch and t1.v_epoch= t2_v_epoch
        group by code_id, obs_epoch, v_epoch ,t2_v_epoch_cnt,sum_v_epoch_cnt


		 
        UNION ALL
        
       select 
            'AVG'::obs_target AS obs_target
            ,obs_epoch
            ,code_id
            ,v_epoch 
            ,coalesce(t2_v_epoch_cnt,0) as v_epoch_cnt
			,coalesce(sum_v_epoch_cnt,0) AS sum_v_epoch_cnt
            ,TRUNC( coalesce(t2_v_epoch_cnt,0) / sum(COUNT(*)) OVER ( PARTITION BY CODE_ID, obs_epoch  )  *100, 2)  as rate_all_epoch
			,coalesce(TRUNC( t2_v_epoch_cnt  / sum_v_epoch_cnt  *100, 2),0)  as rate_epoch
            
        from agg.vol_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
				,COUNT(*)  AS t2_v_epoch_cnt
              	,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.vol_data   where rank_avg =1
            group by code_id, obs_epoch, v_epoch ,rank_avg 
        ) t2 on t1.code_id= t2_code_id and t1.obs_epoch= t2_obs_epoch and t1.v_epoch= t2_v_epoch
        group by code_id, obs_epoch, v_epoch ,t2_v_epoch_cnt,sum_v_epoch_cnt
    )select  code_id
            ,obs_target
            ,obs_epoch
            ,v_epoch 
            ,v_epoch_cnt
            ,sum_v_epoch_cnt
            ,rate_all_epoch
			,rate_epoch
            ,RANK() OVER ( 
                PARTITION BY code_id, obs_epoch, obs_target
                ORDER BY  rate_epoch  DESC
            ) AS RANK_NUM
        from tb_sub 
;    
        
    COMMIT;
    ------------------------------`````````
    ------------------------------
    ------------------------------
    END;
    $$;
-- CALL agg.insert_vol();

/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
                             end CALL agg.insert_vol()
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/

/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_AGG_VOL
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
--drop view PUBLIC.view_agg_vol_freq;
CREATE OR REPLACE VIEW PUBLIC.view_agg_vol_freq AS
SELECT
	C.ID AS CODE_ID,
	C.STD_CODE,
	C.CODE,
	C.NAME,
	C.COUNTRY,
	C.EXCHANGE,
	V.obs_epoch,
	V.obs_target,
	V.v_epoch_cnt,
	V.sum_v_epoch_cnt,
	V.rate_all_epoch,
	V.rate_epoch,
	V.v_obs_agg
FROM
	CODE C
	LEFT JOIN  (
		select 
			code_id, obs_epoch, obs_target, v_epoch_cnt, sum_v_epoch_cnt, rate_all_epoch, rate_epoch, array_agg(v_epoch order by v_epoch asc ) as v_obs_agg
		from agg.vol where rank_num = 1
		group by code_id, obs_epoch, obs_target, v_epoch_cnt, sum_v_epoch_cnt, rate_all_epoch, rate_epoch

    )v  ON C.ID = V.CODE_ID
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_AGG_VOL_BEST END
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/
