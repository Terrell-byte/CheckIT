CREATE TABLE checkitDB.tasks (
    id INT NOT NULL,
    date VARCHAR(255) NOT NULL,
    title VARCHAR(32) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY (id, date)
);
