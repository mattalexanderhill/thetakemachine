CREATE TABLE responses (
  id          SERIAL  PRIMARY KEY,
  session_id  VARCHAR NOT NULL,
  question_id INTEGER NOT NULL REFERENCES questions(id),
  answer_id   INTEGER NOT NULL REFERENCES answers(id),
  at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX responses_session ON responses ( session_id ASC );
