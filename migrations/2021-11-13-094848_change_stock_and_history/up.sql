-- rename count to share
-- share will be in range [0, 1]
-- the smallest number to store will be 0.0001
ALTER TABLE COMMAND DROP COLUMN share;
ALTER TABLE COMMAND ADD COLUMN "count" INTEGER;

ALTER TABLE STOCK ADD COLUMN last_transaction_id;
