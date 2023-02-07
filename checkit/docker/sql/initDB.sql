CREATE TABLE checkitDB.tasks (
    id INT NOT NULL,
    date VARCHAR(255) NOT NULL,
    title VARCHAR(32) NOT NULL,
    descr TEXT NOT NULL,
    PRIMARY KEY (id, date)
);
