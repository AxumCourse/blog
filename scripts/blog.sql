CREATE TABLE categories  (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  is_del BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE topics (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  category_id INT NOT NULL,
  summary VARCHAR(255) NOT NULL,
  markdown VARCHAR NOT NULL,
  html VARCHAR NOT NULL,
  hit INT NOT NULL DEFAULT 0,
  dateline TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  is_del BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (category_id) REFERENCES categories (id)
);

CREATE VIEW v_topic_cat_list AS
  SELECT t.id, title, summary, hit, dateline,category_id,t.is_del,
         c.name AS category_name
    FROM
      topics AS t
      INNER JOIN categories AS c
          ON t.category_id=c.id
                   WHERE c.is_del = false
;

CREATE TABLE admins (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  is_del BOOLEAN NOT NULL DEFAULT FALSE
);
INSERT INTO admins(email,password) VALUES('team@axum.rs', '$2b$12$OljS3FqwxaYXESzu6F0ZRevgBrt9ueY.7NNzdsMOaJk0YoGD5aTii');

CREATE VIEW v_topic_cat_detail AS
  SELECT t.id, title, html, hit, dateline,category_id,t.is_del,
         c.name AS category_name
    FROM
      topics AS t
      INNER JOIN categories AS c
          ON t.category_id=c.id
   WHERE c.is_del = false
         ;
