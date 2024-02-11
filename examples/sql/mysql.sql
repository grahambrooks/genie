-- Create database
CREATE DATABASE IF NOT EXISTS exampleDB;
USE exampleDB;

-- Create tables
CREATE TABLE IF NOT EXISTS Users (
    UserID INT AUTO_INCREMENT,
    UserName VARCHAR(100) NOT NULL,
    UserEmail VARCHAR(100),
    PRIMARY KEY (UserID)
);

CREATE TABLE IF NOT EXISTS Orders (
    OrderID INT AUTO_INCREMENT,
    UserID INT,
    OrderDate DATE,
    PRIMARY KEY (OrderID),
    FOREIGN KEY (UserID) REFERENCES Users(UserID)
);

-- Create view
CREATE VIEW UserOrders AS
SELECT Users.UserName, Orders.OrderDate
FROM Users
JOIN Orders ON Users.UserID = Orders.UserID;

-- Create stored procedure
DELIMITER //
CREATE PROCEDURE InsertUser(IN name VARCHAR(100), IN email VARCHAR(100))
BEGIN
    INSERT INTO Users(UserName, UserEmail) VALUES (name, email);
END //
DELIMITER ;