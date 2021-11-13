-- This file should undo anything in `up.sql`
ALTER TABLE STOCK ADD COLUMN last_transaction_id;
ALTER TABLE COMMAND DROP COLUMN share;
ALTER TABLE COMMAND ADD COLUMN count;