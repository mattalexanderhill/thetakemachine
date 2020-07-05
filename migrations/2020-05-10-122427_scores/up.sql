CREATE TABLE scores (
  id          SERIAL  PRIMARY KEY,
  question_id INTEGER NOT NULL REFERENCES questions(id),
  answer_id   INTEGER NOT NULL REFERENCES answers(id),
  x           INTEGER NULL,
  y           INTEGER NULL
);
