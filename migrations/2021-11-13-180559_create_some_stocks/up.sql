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
BEGIN
   -- fill stonker ids
    SELECT id INTO netflixer from stonker WHERE name = 'netflixer';
    SELECT id INTO jeff from stonker WHERE name = 'jeff';
    SELECT id INTO walt from stonker WHERE name = 'walt';

    -- fill company ids
    SELECT id INTO netflix from company WHERE name = 'netflix';
    SELECT id INTO amazon from company WHERE name = 'amazon';
    SELECT id INTO disney from company WHERE name = 'disney';

    INSERT INTO stock (stonker_id, company_id) VALUES (netflixer, netflix) returning id INTO netflixer;
    INSERT INTO stock (stonker_id, company_id) VALUES (jeff, amazon) returning id INTO netflixer;
    INSERT INTO stock (stonker_id, company_id) VALUES (walt, disney) returning id INTO netflixer;

    RETURN 3;  -- number of changed columns 
END;
$$ LANGUAGE plpgsql;
SELECT generate_data();
DROP FUNCTION generate_data();
