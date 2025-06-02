Feature: Post Vocabulary Items

  Background:
    Given a clean database is available
    And the server is started

  Scenario: Add one Translation
    When I add a complete translation
    Then the http response is "OK"

  Scenario: Add one Translation
    When I add a complete translation
    Then the same translation record is returned

  Scenario: Add same Translation twice
    When I add a complete translation
    And I add a complete translation
    Then the http response class is "Client Error"
    And the http response is "CONFLICT"
