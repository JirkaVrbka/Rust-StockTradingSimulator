-- Your SQL goes here
CREATE OR REPLACE FUNCTION generate_data() RETURNS int AS $$
DECLARE
    -- stock ids
	netflix_stock1 int;
    netflix_stock2 int;
    netflix_stock3 int;
    amazon_stock1 int;
    amazon_stock2 int;
    amazon_stock3 int;
    disney_stock1 int;
    disney_stock2 int;
    disney_stock3 int;
    -- company ids
    netflix int;
    amazon int;
    disney int;
    -- stonker ids
    netflixer int;
    jeff int;
    walt int;
BEGIN
    DELETE FROM STOCK;

    -- Fill company ids
    SELECT id INTO netflix from company WHERE name = 'netflix';
    SELECT id INTO amazon from company WHERE name = 'amazon';
    SELECT id INTO disney from company WHERE name = 'disney';

   -- fill stonker ids
    SELECT id INTO netflixer from stonker WHERE name = 'netflixer';
    SELECT id INTO jeff from stonker WHERE name = 'jeff';
    SELECT id INTO walt from stonker WHERE name = 'walt';

    -- Create netflix stocks for selling
	INSERT INTO stock (stonker_id, company_id) VALUES (netflixer, netflix) returning id INTO netflix_stock1;
    INSERT INTO stock (stonker_id, company_id) VALUES (netflixer, netflix) returning id INTO netflix_stock2;
    INSERT INTO stock (stonker_id, company_id) VALUES (netflixer, netflix) returning id INTO netflix_stock3;

    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (netflixer, netflix_stock1, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (netflixer, netflix_stock2, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (netflixer, netflix_stock3, 0, NOW(), NULL);

    -- Offer netflix stocks
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (netflixer, netflix, 300, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (netflixer, netflix, 250, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (netflixer, netflix, 325, 1, 'SELL');

     -- Create amazon stocks for selling
    INSERT INTO stock (stonker_id, company_id) VALUES (jeff, amazon) returning id INTO amazon_stock1;
    INSERT INTO stock (stonker_id, company_id) VALUES (jeff, amazon) returning id INTO amazon_stock2;
    INSERT INTO stock (stonker_id, company_id) VALUES (jeff, amazon) returning id INTO amazon_stock3;

    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (jeff, amazon_stock1, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (jeff, amazon_stock2, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (jeff, amazon_stock3, 0, NOW(), NULL);

    -- Offer amazon stocks
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (jeff, amazon, 213, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (jeff, amazon, 258, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (jeff, amazon, 301, 1, 'SELL');

     -- Create disney stocks for selling
    INSERT INTO stock (stonker_id, company_id) VALUES (walt, disney) returning id INTO disney_stock1;
    INSERT INTO stock (stonker_id, company_id) VALUES (walt, disney) returning id INTO disney_stock2;
    INSERT INTO stock (stonker_id, company_id) VALUES (walt, disney) returning id INTO disney_stock3;

    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (walt, disney_stock1, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (walt, disney_stock2, 0, NOW(), NULL);
    INSERT INTO history (stonker_id, stock_id, bought_for, created_at, sold_for) VALUES (walt, disney_stock3, 0, NOW(), NULL);

    -- Offer disney stocks
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (walt, disney, 123, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (walt, disney, 200, 1, 'SELL');
    INSERT INTO command (stonker_id, company_id, threshold, share, kind) VALUES (walt, disney, 321, 1, 'SELL');

    RETURN 1;
END;
$$ LANGUAGE plpgsql;
SELECT generate_data();
DROP FUNCTION generate_data();