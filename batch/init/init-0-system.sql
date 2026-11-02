CREATE SCHEMA IF NOT EXISTS SYSTEM;
CREATE TABLE IF NOT EXISTS SYSTEM.BATCH (
    id BIGSERIAL PRIMARY KEY,
    start_time timestamptz,
    end_time timestamptz,
    run_time_sec INTERVAL
);
CREATE TABLE IF NOT EXISTS SYSTEM.BATCH_STEP (
    id BIGSERIAL PRIMARY KEY,
    batch_id BIGINT,
    job_name VARCHAR,
    step_name VARCHAR,
    start_time timestamptz,
    end_time timestamptz,
    run_time_sec INTERVAL,
    FOREIGN KEY (batch_id) REFERENCES SYSTEM.BATCH (id)
);
CREATE TABLE IF NOT EXISTS SYSTEM.BATCH_ERROR (
    batch_id BIGINT,
    batch_step_id BIGINT,
    content JSON,
    created_at timestamptz DEFAULT now(),
    FOREIGN KEY (batch_id) REFERENCES SYSTEM.BATCH (id),
    FOREIGN KEY (batch_step_id) REFERENCES SYSTEM.BATCH_STEP (id)
);
CREATE TABLE IF NOT EXISTS SYSTEM.INFO (id int UNIQUE, name VARCHAR, dt timestamptz);
INSERT INTO system.INFO(id, name, dt)
VALUES (1, 'health-batch_timer', '2000-01-01');
CREATE TYPE batch_type AS enum('BATCH', 'NEWS');
CREATE TYPE run_type AS enum('START', 'EXIT');
CREATE TABLE IF NOT EXISTS SYSTEM.BATCH_TIMER_PLAN (
    id BIGSERIAL PRIMARY KEY,
    batch_type batch_type,
    country_type country_type,
    run_type run_type,
    updated_dt timestamptz,
    crated_dt timestamptz DEFAULT now()
);