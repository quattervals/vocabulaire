Feature: Database operations

  Background:
    Given a clean database is available
    And the server is started

  Scenario: Add one Translation
    When I add a complete translation
    Then the operation should succeed

  Scenario: Add same Translation twice
    When I add a complete translation
    And I add a complete translation
    Then the opration is a client error
    And is duplicate
