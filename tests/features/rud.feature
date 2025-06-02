Feature: Read, Update, Delete Vocabulary Items

  Background:
    Given a clean database is available
    And the server is started
    And I add a complete translation

  Scenario: Reading a Translation
    When I read existing word
    Then I receive the corresponding TranslationRecord
    And the http response is "OK"
