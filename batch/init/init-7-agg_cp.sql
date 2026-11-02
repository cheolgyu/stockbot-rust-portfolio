

/*
 ==========================================================================================================
 ==========================================================================================================
 agg.cp_data
 ==========================================================================================================
 ==========================================================================================================
*/
    --drop table agg.cp_data CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.cp_data (
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
        CONSTRAINT agg_cp_data_pk PRIMARY KEY(code_id, year, obs_epoch, v_epoch )
    );
    --drop table agg.cp CASCADE ;
    CREATE TABLE IF NOT EXISTS agg.cp (
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
        CONSTRAINT agg_cp_pk PRIMARY KEY(code_id, obs_epoch, v_epoch, obs_target )
    );
/*
 ==========================================================================================================
 ==========================================================================================================
 PROCEDURE agg.insert_cp_data()
 Query returned successfully in 2 min 5 secs.
 ==========================================================================================================
 ==========================================================================================================
*/
 -- drop PROCEDURE agg.insert_cp_data();
    CREATE PROCEDURE agg.insert_cp_data() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.cp_data ;

    INSERT INTO agg.cp_data(code_id, year, obs_epoch, v_epoch, v_target_avg, v_target_min, v_target_max, rank_avg, rank_min, rank_max)
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
            TRUNC(avg(cp),2) as v_target_avg, 
            min(cp)as v_target_min, 
            max(cp) as v_target_max 
        from hist.price
        GROUP BY code_id, year, v_epoch 
        UNION ALL
        select 
            'WEEK'::obs_epoch AS obs_epoch,
            code_id,
            EXTRACT(year FROM dt) AS year,
            EXTRACT(WEEK FROM dt) AS v_epoch,
            TRUNC(avg(cp),2) as v_target_avg, 
            min(cp)as v_target_min, 
            max(cp) as v_target_max 
        from hist.price
        GROUP BY code_id, year, v_epoch 
        UNION ALL
        select 
            'QUARTER'::obs_epoch AS obs_epoch,
            code_id,
            EXTRACT(year FROM dt) AS year,
            EXTRACT(QUARTER FROM dt) AS v_epoch,
            TRUNC(avg(cp),2) as v_target_avg, 
            min(cp)as v_target_min, 
            max(cp) as v_target_max 
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

-- call agg.insert_cp_data();

/*
 ==========================================================================================================
 ==========================================================================================================
 
 PROCEDURE agg.insert_cp()
Query returned successfully in 26 secs 782 msec.

 ==========================================================================================================
 ==========================================================================================================
*/
-- drop PROCEDURE agg.insert_cp();
    CREATE PROCEDURE agg.insert_cp() LANGUAGE plpgsql AS $$ ---
    BEGIN ------------------------------
    truncate table agg.cp ;
           
    INSERT INTO agg.cp(
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
            
        from agg.cp_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
                , COUNT(*)  AS t2_v_epoch_cnt
				,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.cp_data  where rank_max =1
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
            
        from agg.cp_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
				,COUNT(*)  AS t2_v_epoch_cnt
                ,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.cp_data   where rank_min =1
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
            
        from agg.cp_data t1 left join (
            select 
                obs_epoch as t2_obs_epoch
                ,code_id as t2_code_id
                ,v_epoch as t2_v_epoch
				,COUNT(*)  AS t2_v_epoch_cnt
              	,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
            from agg.cp_data   where rank_avg =1
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
-- CALL agg.insert_cp();
/*
 ==========================================================================================================
 ==========================================================================================================
 agg.cp_data
 ==========================================================================================================
 ==========================================================================================================
*/

--drop view PUBLIC.view_agg_cp_freq;
CREATE OR REPLACE VIEW PUBLIC.view_agg_cp_freq AS
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
		from agg.cp where rank_num = 1
		group by code_id, obs_epoch, obs_target, v_epoch_cnt, sum_v_epoch_cnt, rate_all_epoch, rate_epoch

    )v  ON C.ID = V.CODE_ID
/*
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
    VIEW_AGG_cp_BEST END
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------
*/


