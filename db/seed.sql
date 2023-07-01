INSERT INTO game (id, name)
VALUES
    ('00000000-0000-0000-0000-000000000000', 'Sample');

INSERT INTO team (game, id, name, access_code)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'Team Great', 'great'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'Team Awesome', 'awesome');

INSERT INTO widget (game, id, ident, priority, config)
VALUES
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'intro', 0, '{
        "type": "text",
        "heading": "Intro",
        "content": ["Welcome!"],
        "visible": "always"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', 'first', 10, '{
        "type": "prompt",
        "name": "First",
        "details": ["This is the first cipher.", "Please solve it and submit the solution."],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "first"},
            {"type": "number", "solution": 1}
        ],
        "visible": "2022-01-01 12:00 +2",
        "on_solution_correct": "Yup!",
        "on_solution_incorrect": "Nope!"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000002', 'move', 15, '{
        "type": "text",
        "heading": "Instructions",
        "content": ["Please go to the second cipher"],
        "visible": "first.solved"
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000003', 'second', 20, '{
        "type": "prompt",
        "name": "Second",
        "details": ["Good luck with the second cipher!"],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "one"},
            {"type": "alphanumeric", "solution": "two"}
        ],
        "solution_exclusion_group": "main",
        "visible": "first.solved",
        "on_solution_incorrect": "Wrong solution."
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000004', 'third', 30, '{
        "type": "prompt",
        "name": "Third",
        "details": [],
        "prompt": "Answer:",
        "submit_button": "Submit",
        "solutions": [
            {"type": "alphanumeric", "solution": "one"},
            {"type": "alphanumeric", "solution": "two"}
        ],
        "solution_exclusion_group": "main",
        "visible": "(second.visible + 15 s) | second.solved",
        "on_solution_incorrect": "Wrong solution."
    }'),
    ('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000005', 'countdown', 40, '{
        "type": "countdown",
        "name": "This is a countdown",
        "details": ["What will happen?"],
        "done_text": "Done!",
        "time": "(second.solved + 10h) | (third.solved + 30s)",
        "visible": "first.solved"
    }');
