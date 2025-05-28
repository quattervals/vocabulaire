Feature: Database operations
  Background:
    Given a clean database is available
    And the server is started

  Scenario: Basic database operation
    When I add something
    Then the operation should succeed
