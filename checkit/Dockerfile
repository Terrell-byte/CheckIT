# Use the official MariaDB image as the base image
FROM mariadb:latest

# Set the environment variables for the database
ENV MYSQL_DATABASE=mydb
ENV MYSQL_USER=myuser
ENV MYSQL_PASSWORD=mypassword
ENV MYSQL_ROOT_PASSWORD=rootpassword

# Copy the database initialization script to the Docker image
COPY initDB.sql /docker-entrypoint-initdb.d/

# Expose the MariaDB port for external connections
EXPOSE 3306