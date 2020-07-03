CREATE TABLE gender (
  id   INTEGER PRIMARY KEY,
  text VARCHAR NOT NULL
);

INSERT INTO gender (
  id,
  text
)
VALUES
  (1, 'Woman'),
  (2, 'Man'),
  (3, 'Other')
;

CREATE TABLE age_group (
  id   INTEGER PRIMARY KEY,
  text VARCHAR NOT NULL
);

INSERT INTO age_group (
  id,
  text
)
VALUES
  (1, '<25'),
  (2, '25-45'),
  (3, '45-45'),
  (4, '45-55'),
  (5, '55-65'),
  (6, '65<')
;

CREATE TABLE politics (
  id   INTEGER PRIMARY KEY,
  text VARCHAR NOT NULL
);

INSERT INTO politics (
  id,
  text
)
VALUES
  (1, 'Ineligible'),
  (2, 'Labour'),
  (3, 'Conservative'),
  (4, 'Liberal Democrats'),
  (5, 'Green'),
  (6, 'Brexit'),
  (7, 'Scottish National Party'),
  (8, 'Other'),
  (9, 'Did not vote')
;

CREATE TABLE ethics (
  id   INTEGER PRIMARY KEY,
  text VARCHAR NOT NULL
);

INSERT INTO ethics (
  id,
  text
)
VALUES
  (1, 'Co-operate'),
  (2, 'Defect'),
  (3, 'Don''t know')
;

CREATE TABLE demographics (
  id         SERIAL PRIMARY KEY,
  session_id VARCHAR NOT NULL,
  gender     INTEGER NULL REFERENCES gender(id),
  age_group  INTEGER NULL REFERENCES age_group(id),
  politics   INTEGER NULL REFERENCES politics(id),
  ethics     INTEGER NULL REFERENCES ethics(id)
);
