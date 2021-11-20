CREATE OR REPLACE FUNCTION generate_data() RETURNS int AS $$
DECLARE
   -- stonker ids
	netflixer int;
    jeff int;
    walt int;
   -- company ids
    netflix int;
    amazon int;
    disney int;
     -- stonker ids
    stonkeris int;
BEGIN
   -- fill stonker ids
    SELECT id INTO netflixer from stonker WHERE name = 'netflixer';
    SELECT id INTO jeff from stonker WHERE name = 'jeff';
    SELECT id INTO walt from stonker WHERE name = 'walt';

    -- fill company ids
    SELECT id INTO netflix from company WHERE name = 'netflix';
    SELECT id INTO amazon from company WHERE name = 'amazon';
    SELECT id INTO disney from company WHERE name = 'disney';

    -- Offer stocks
    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (netflixer, netflix, 300, 1, 'SELL', NOW());
    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (netflixer, netflix, 325, 1, 'SELL', NOW());

    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (jeff, amazon, 200, 1, 'SELL', NOW());
    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (jeff, amazon, 225, 1, 'SELL', NOW());

    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (walt, disney, 100, 1, 'SELL', NOW());
    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (walt, disney, 125, 1, 'SELL', NOW());

    -- Create big stonker
    INSERT INTO STONKER ("name", balance, blocked_balance, invested_balance) VALUES ('stonkeris', 500, 50, 675) returning id INTO stonkeris;

    -- Create delayed action - buy disney 0.0001 share if the cost is lower than 50
    INSERT INTO command (stonker_id, company_id, threshold, share, kind, created_at) VALUES (stonkeris, disney, 50, 1, 'BUY_IF_LOW', NOW());

    -- Buy stocks
    INSERT INTO stock (stonker_id, company_id, share, bought_for) VALUES (stonkeris, netflix, 1, 300);
    INSERT INTO stock (stonker_id, company_id, share, bought_for) VALUES (stonkeris, amazon, 1, 225);
    INSERT INTO stock (stonker_id, company_id, share, bought_for) VALUES (stonkeris, disney, 1, 150);

    RETURN 3;
END;
$$ LANGUAGE plpgsql;
SELECT generate_data();
DROP FUNCTION generate_data();