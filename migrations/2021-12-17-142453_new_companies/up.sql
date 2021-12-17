CREATE OR REPLACE FUNCTION generate_data() RETURNS int AS $$
DECLARE
    marc int;
    marty int;

BEGIN
    INSERT INTO stonker (name, balance, blocked_balance, invested_balance) VALUES ('marc', 500, 0, 0) returning id INTO marc;
    INSERT INTO company (name, performer_id) VALUES ('Netscape', marc);

    INSERT INTO stonker (name, balance, blocked_balance, invested_balance) VALUES ('marty', 800, 0, 0) returning id INTO marty;
    INSERT INTO company (name, performer_id) VALUES ('Networld', marty);

    RETURN 4;  -- number of changed columns
END;
$$ LANGUAGE plpgsql;
SELECT generate_data();
DROP FUNCTION generate_data();

