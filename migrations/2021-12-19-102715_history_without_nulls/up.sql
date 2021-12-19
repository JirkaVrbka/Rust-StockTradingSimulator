-- Stock moves to history after transaction is complete, there's no need for null fields
ALTER TABLE history ALTER COLUMN bought_for SET NOT NULL;
ALTER TABLE history ALTER COLUMN sold_for SET NOT NULL;
