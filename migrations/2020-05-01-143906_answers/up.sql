CREATE TABLE answers (
  id   INTEGER PRIMARY KEY,
  text VARCHAR NOT NULL
);

INSERT INTO answers (
  id,
  text
)
VALUES
  (1, 'Agree'),
  (2, 'Disagree'),
  (3, 'Don''t know'),
  (4, 'Don''t care')
;