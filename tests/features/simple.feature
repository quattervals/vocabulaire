Feature: Animal feature

  Scenario: If we feed a hungry cat it will no longer be hungry
    Given a hungry cat
    When I feed the cat
    Then the cat is not hungry


  Scenario: If we feed a satiated cat it will not become hungry
    Given a satiated cat
    When I feed the cat
    Then the cat is not hungry

    # #[test]
    # fn word_new_ok_input_constructed() {
    #     let word = Word::new("chien".to_string(), Lang::fr);
    #     assert_eq!((word.unwrap().value()), (&"chien".to_string(), &Lang::fr));
    # }
