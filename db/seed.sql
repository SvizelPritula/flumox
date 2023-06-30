INSERT INTO game (id, name)
VALUES
    ('00000000-0000-0000-0000-000000000000', 'Sample');

INSERT INTO team (game, id, name, access_code)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'Team Great', 'great'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'Team Awesome', 'awesome');

INSERT INTO widget (game, id, ident, priority, config)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'first', 10, '{
        "type": "prompt",
        "name": "First",
        "details": ["This is the first cipher.", "Please solve it and submit the solution."],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "first"},
            {"type": "number", "solution": 1}
        ],
        "visible": "2022-01-01 12:00 +2"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'second', 20, '{
        "type": "prompt",
        "name": "Second",
        "details": ["Good luck with the second cipher!"],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "second"},
            {"type": "alphanumeric", "solution": "two"}
        ],
        "visible": "first.solved"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000002', 'third', 30, '{
        "type": "prompt",
        "name": "Third",
        "details": [],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "one two three"}
        ],
        "visible": "(second.visible + 15 s) | second.solved"
    }');
