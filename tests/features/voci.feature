Feature: Database operations

  Background:
    Given a clean database is available
    And the server is started

  Scenario: Add one Translation
    When I add a complete translation
    Then the operation should succeed
