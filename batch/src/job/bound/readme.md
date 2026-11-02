- 실행 순서
  - 바운스 만들고 hist 테이블에 저장
    - 코드별 바운스 시작일 조회
    - 코드별 가격종류 별
      - 시작일 이후 가격목록, 가격tick 조회
        - 기존: 가격 tick 은 hist.price 저장시 trigger 로 생성. X_TICK 도 트리거에서 생성 .
          - 트리거는 일자를 테이블에서 저장하고 PRICE STEP 마지막 STEP 에서 랭킹으로 테이블 UPDATE를 함.
        ```
            BEGIN IF (TG_OP = 'INSERT') THEN
            SELECT exchange INTO var_exchange FROM code WHERE id = NEW.code_id;
            INSERT INTO utils.convert_date(exchange, dt)
            VALUES (var_exchange, NEW.dt) ON CONFLICT (exchange, dt) DO NOTHING;
            SELECT id INTO var_date_point_id
            FROM utils.convert_date
            WHERE exchange = var_exchange
                AND dt = NEW.dt;
        ```
        - 변경: 가격 tick 은 bound 에서 조인으로 
      - 바운드 생성
      - 바운드 저장
  - 바운드 데이일리 저장