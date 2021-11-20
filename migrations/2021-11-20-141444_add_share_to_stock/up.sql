-- Your SQL goes here
ALTER TABLE STOCK ADD COLUMN "share" INTEGER;

UPDATE STOCK SET share=1;

ALTER TABLE STOCK ALTER COLUMN "share" SET NOT NULL;
ALTER TABLE STOCK ADD CONSTRAINT stock_share_is_positive CHECK (share > 0);