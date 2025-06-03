Feature: Read, Update, Delete Vocabulary Items

  Background:
    Given a clean database is available
    And the server is started
    And there is a translation

  Scenario: Reading a Translation
    When I read an existing word
    Then the corresponding TranslationRecord is received
    And the http response is "OK"

  Scenario: Deleting a Translation
    When I delete an existing translation
    Then the http response is "OK"

  Scenario: Deleting a non existing Translation
    When I delete a non-existing translation
    Then the http response is "BAD_REQUEST"

  Scenario: Update an existing Translation
    When I update an existing translation
    Then the updated TranslationRecord is received
    And the http response is "OK"

  Scenario: Update an existing Translation
    When I update a non-existing translation
    Then the http response is "BAD_REQUEST"
