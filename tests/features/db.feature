Feature: Database operations
  Background:
    Given a clean database is available

  Scenario: Basic database operation
    When I perform a database operation
    Then the operation should succeed
