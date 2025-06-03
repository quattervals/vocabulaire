Feature: Create Vocabulary Items

  Background:
    Given a clean database is available
    And the server is started

  Scenario: Add one Translation
    When I create a sound translation item
    Then the http response is "OK"

  Scenario: Add one Translation
    When I create a sound translation item
    Then the corresponding TranslationRecord is received

  Scenario: Add same Translation twice
    When I create a sound translation item
    And I create a sound translation item
    Then the http response class is "Client Error"
    And the http response is "CONFLICT"
