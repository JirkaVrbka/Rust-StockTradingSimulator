-- Your SQL goes here
DELETE FROM HISTORY;
DELETE FROM STOCK;

ALTER TABLE STOCK ADD COLUMN bought_for INTEGER NOT NULL;
ALTER TABLE STOCK ADD COLUMN sold_for INTEGER;