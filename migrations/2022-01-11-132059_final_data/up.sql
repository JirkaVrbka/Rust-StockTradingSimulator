TRUNCATE Stonker RESTART IDENTITY CASCADE;
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Atlas Crest Investment Corp. II Class A', 'MXANU', 57045, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Murphy USA Inc.', 'y81%j', 25975, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Black Hills Corporation', '#XaaL', 62025, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Carnival Corporation', 'LJi&e', 16551, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Clover Leaf Capital Corp. Class A', '9l1qD', 41664, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Vivian Davis', 'UVfNq', 43563, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Shane Evans', '7ArYk', 25968, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Gloria Holland', 'FxV3u', 2269, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Joey Butler', 'E#PBa', 83583, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Greyson Singh', 'eT2e5', 65962, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Gemma Jones', '#Nq46', 44747, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Orlando Butler', '4d)xw', 87084, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Liam Ward', '^U)DB', 26622, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Harmony Powell', 'o%QKl', 53153, 0, 0);
INSERT INTO Stonker (name, password, balance, blocked_balance, invested_balance) VALUES ('Albert Crawford', 'ka@WJ', 75875, 0, 0);

TRUNCATE Company RESTART IDENTITY CASCADE;
INSERT INTO Company (name, performer_id) VALUES ('ACII', 1);
INSERT INTO Company (name, performer_id) VALUES ('MUSA', 2);
INSERT INTO Company (name, performer_id) VALUES ('BKH', 3);
INSERT INTO Company (name, performer_id) VALUES ('CCL', 4);
INSERT INTO Company (name, performer_id) VALUES ('CLOE', 5);

TRUNCATE News RESTART IDENTITY CASCADE;
INSERT INTO News (title, description, author, created_at, kind, company_id) VALUES ('Position of CCL', 'Clinton indon visit monday. Future of Carnival Corporation is not clear.', 'Vedomosti', '2022-01-08 23:27:12', 'NEUTRAL', 4);
INSERT INTO News (title, description, author, created_at, kind, company_id) VALUES ('Strong Sell BKH', 'Property group claims easter trading laws unfair. Investors of Black Hills Corporation are quite happy with the current situation.', 'El Economista', '2022-01-09 13:33:02', 'RISE', 3);
INSERT INTO News (title, description, author, created_at, kind, company_id) VALUES ('Fall of BKH', 'Visitor info centre faces funding crisis. Future of Black Hills Corporation is looking pretty bad.', 'The Nikkei', '2022-01-10 12:02:53', 'FALL', 3);

