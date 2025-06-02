Feature: Read, Update, Delete Vocabulary Items

  Background:
    Given a clean database is available
    And the server is started
    And there is a translation

  Scenario: Reading a Translation
    When I read an existing word
    Then the corresponding TranslationRecord is received
    And the http response is "OK"
