/* 
========================================
Query complete 00:00:23.742
Query complete 00:01:27.095


========================================
*/
--drop function agg.select_agg_bound_data
CREATE or replace FUNCTION agg.select_agg_bound_data( inp_obs_epoch TEXT,  inp_agg_bound_type TEXT,  inp_agg_col TEXT ) --
RETURNS  TABLE (code_id bigint, year numeric, v_epoch numeric, obs_epoch obs_epoch, agg_bound_type agg_bound_type
, v_target_avg numeric, v_target_min numeric, v_target_max numeric
, rank_avg numeric, rank_MIN numeric, rank_max numeric)
  AS $$
BEGIN 
RETURN query EXECUTE  format(' 
select 
	code_id,
	year,
	v_epoch,
	obs_epoch,
	agg_bound_type,
	v_target_avg, 
	v_target_min, 
	v_target_max 	
	,rank() OVER (PARTITION BY code_id, year, obs_epoch   ORDER BY v_target_avg DESC)::numeric as rank_avg
	,rank() OVER (PARTITION BY code_id, year, obs_epoch   ORDER BY v_target_min ASC)::numeric as rank_MIN
	,rank() OVER (PARTITION BY code_id, year, obs_epoch   ORDER BY v_target_max DESC)::numeric as rank_max
from (
 	select 
		code_id,
		''%2$s''::agg_bound_type AS agg_bound_type,
		''%1$s''::obs_epoch AS obs_epoch,
		EXTRACT(year FROM p1x) AS year,
		EXTRACT(%1$s FROM p1x) AS v_epoch,
		TRUNC(avg(%3$s),2)::numeric  as v_target_avg, 
		min(%3$s)::numeric  as v_target_min, 
		max(%3$s)::numeric  as v_target_max 
		
	from hist.bound
	GROUP BY code_id, year, v_epoch   
) 
 ;',inp_obs_epoch, inp_agg_bound_type, inp_agg_col);

END;
$$ LANGUAGE plpgsql;
select * from agg.select_agg_bound_data('MONTH','CHG','change_rate');
select * from agg.select_agg_bound_data('MONTH','DAY','x_cnt');

/* 
========================================
Query complete 00:00:12.094

========================================
*/
--drop function agg.select_agg_bound;
CREATE or replace FUNCTION agg.select_agg_bound( inp_obs_target TEXT, inp_agg_bound_type TEXT,  inp_col TEXT) --
RETURNS  TABLE (code_id bigint, obs_epoch obs_epoch, v_epoch numeric , obs_target obs_target, agg_bound_type agg_bound_type
, v_epoch_cnt numeric, sum_v_epoch_cnt numeric, rate_all_epoch numeric , rate_epoch numeric, RANK_NUM numeric )
  AS $$
BEGIN 
RETURN query EXECUTE  format(' 
with t as (
	select 
		obs_epoch as t2_obs_epoch
		,code_id as t2_code_id
		,v_epoch as t2_v_epoch
		, COUNT(*)  AS t2_v_epoch_cnt
		,sum(COUNT(*) ) OVER ( PARTITION BY CODE_ID, obs_epoch  ) AS sum_v_epoch_cnt
	from agg.bound_data  where  %3$s =1 AND agg_bound_type=''%2$s''
	group by code_id, obs_epoch, v_epoch ,%3$s 
), T2 AS(
  select 
		code_id
		,obs_epoch
		,v_epoch
		,''%1$s''::obs_target AS obs_target
		,''%2$s''::agg_bound_type AS agg_bound_type
		 
		,coalesce(t2_v_epoch_cnt,0)::numeric as v_epoch_cnt
		,coalesce(sum_v_epoch_cnt,0)::numeric AS sum_v_epoch_cnt
		,TRUNC( coalesce(t2_v_epoch_cnt,0) / sum(COUNT(*)) OVER ( PARTITION BY CODE_ID, obs_epoch  )  *100, 2)::numeric  as rate_all_epoch
		,coalesce(TRUNC( t2_v_epoch_cnt  / sum_v_epoch_cnt  *100, 2),0)::numeric  as rate_epoch
		
	from agg.bound_data t1 left join t as t2 on t1.code_id= t2_code_id and t1.obs_epoch= t2_obs_epoch and t1.v_epoch= t2_v_epoch
	where agg_bound_type=''%2$s''
	group by code_id, obs_epoch, v_epoch, t2_v_epoch_cnt, sum_v_epoch_cnt 
)
select  
	code_id
	,obs_epoch
	,v_epoch 
	,obs_target
	,agg_bound_type
	,v_epoch_cnt
	,sum_v_epoch_cnt
	,rate_all_epoch
	,rate_epoch
	,RANK() OVER ( 
		PARTITION BY code_id, obs_epoch, obs_target
		ORDER BY  rate_epoch  DESC
	) ::numeric AS RANK_NUM
FROM T2

 ;',inp_obs_target, inp_agg_bound_type, inp_col);
END;
$$ LANGUAGE plpgsql;


select * from agg.select_agg_bound('MIN','CHG','rank_min');
select * from agg.select_agg_bound('MIN','DAY','rank_min');
select * from agg.select_agg_bound('MAX','CHG','rank_MAX');
select * from agg.select_agg_bound('MAX','DAY','rank_MAX');
select * from agg.select_agg_bound('AVG','CHG','rank_AVG');
select * from agg.select_agg_bound('AVG','DAY','rank_AVG');