UPDATE news SET
    kind='FALL',
    company_id=1
where id=1;

UPDATE news SET
    kind='RISE',
    company_id=4
where id=2;

UPDATE news SET
    kind='NEUTRAL',
    company_id=5
where id=3;

ALTER TABLE news ALTER COLUMN kind SET NOT NULL;
ALTER TABLE news ALTER COLUMN company_id SET NOT NULL;