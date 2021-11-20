-- Your SQL goes here
CREATE TABLE NEWS (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    "description" TEXT NOT NULL,
    author VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

INSERT INTO NEWS (title, "description", author, created_at) VALUES ('Netflix in Flames', 'After a sudden change
in leadership, all stocks
drop to almost zero!', 'The Devil', NOW());

INSERT INTO NEWS (title, "description", author, created_at) VALUES ('Netscape', 'We all thought that
this browser died
in year 2006, but
it is in fact quality
stock to invest in.', 'Hopers', NOW());

INSERT INTO NEWS (title, "description", author, created_at) VALUES ('Networld', 'This might be the
next Netscape.
We will update you
as soon as stock
market opens.', 'Infoworld', NOW());