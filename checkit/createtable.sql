CREATE TABLE IF NOT EXIST `tasks` (
  `id` varchar(255) NOT NULL,
  `title` varchar(255) DEFAULT NULL,
  `description` varchar(255) DEFAULT NULL,
  `isCompleted` BIT(2) DEFAULT 0,
  PRIMARY KEY (`id`)
)