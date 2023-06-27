INSERT INTO game (id, name)
VALUES
    ('00000000-0000-0000-0000-000000000000', 'Sample');

INSERT INTO team (game, id, name, access_code)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'Team Great', 'great'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'Team Awesome', 'awesome');

INSERT INTO widget (game, id, ident, config)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'first', '{
        "type": "prompt",
        "name": "First",
        "details": ["This is the first cipher.", "Please solve it and submit the solution."],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [],
        "visible": "2022-01-01 12:00 +2"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'second', '{
        "type": "prompt",
        "name": "Second",
        "details": ["Good luck with the second cipher!"],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [],
        "visible": "first.solved"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000002', 'third', '{
        "type": "prompt",
        "name": "Third",
        "details": [],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [],
        "visible": "(second.visible + 10 m) | second.solved"
    }');

INSERT INTO state (game, team, widget, state)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', '{"type": "prompt", "solved": {"time": "2022-01-01T12:31:00.00+02:00", "canonical_text": ""}}'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', '{"type": "prompt", "solved": null}'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', '{"type": "prompt", "solved": null}');
