-- Your SQL goes here
INSERT INTO stonker (name, balance) VALUES ('netflixer', 5000) returning id INTO netflixer;
INSERT INTO company (name, performer_id) VALUES ('netflix', netflixer);

INSERT INTO stonker (name, balance) VALUES ('jeff', 8888) returning id INTO jeff;
INSERT INTO company (name, performer_id) VALUES ('amazon', jeff);

INSERT INTO stonker (name, balance) VALUES ('walt', 70) returning id INTO walt;
INSERT INTO company (name, performer_id) VALUES ('disney', walt);


