
 
-- 로그테이블 task 테이블
CREATE TABLE IF NOT EXISTS log.task (
    tag log_task_tag NOT NULL,
    log_type log_task_type NOT NULL,
    msg json NOT NULL,
    created_at timestamp NOT NULL
);
CREATE INDEX IF NOT EXISTS created_at_idx ON log.task (created_at);