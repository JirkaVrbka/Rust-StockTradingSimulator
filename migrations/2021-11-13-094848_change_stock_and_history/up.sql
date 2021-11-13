-- rename count to share
-- share will be in range [0, 1]
-- the smallest number to store will be 0.0001
ALTER TABLE COMMAND DROP COLUMN count;
ALTER TABLE COMMAND ADD COLUMN share NUMERIC(5,4);

ALTER TABLE STOCK DROP COLUMN last_transaction_id;
