INSERT INTO game (id, name)
VALUES
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', 'Ukázková hra');

INSERT INTO team (game, id, name, access_code, attributes)
VALUES
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '37f38c56-7bde-4fc5-b322-83eff8801dd8', 'Kyborgové 1', 'absurd-opportunity-1', '{}'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '7c29794a-1ce3-4daf-9e2c-ac38d26ada81', 'Kyborgové 2', 'absurd-opportunity-2', '{}'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', 'dc4d3aec-e341-4603-b418-04a15a4b8be9', 'Kyborgové 3', 'absurd-opportunity-3', '{}'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '2e5db16d-74c3-4e7a-8ab7-5a2194140a2b', 'Kyborgové 4', 'absurd-opportunity-4', '{}'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', 'b4ca5939-b5ef-4e1c-88d9-0c0272377f7a', 'Kyborgové 5', 'absurd-opportunity-5', '{}');

INSERT INTO widget (game, id, ident, priority, config)
VALUES
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '3b284529-9164-42da-847d-c3fadf1d125a', 'intro', 0, '{
        "type": "text",
        "heading": "Něco ke hře",
        "content": ["Cílem této hry je představit webovou aplikaci Flumox. Řešení všech \"šifer\" v této hře najdete v textu."],
        "visible": "always",
        "obsolete": "to-start.solved"
    }'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '4bcd62b0-c2ed-46f2-b3c5-5a324573372c', 'to-start', 10, '{
        "type": "prompt",
        "name": "Cesta na start",
        "details": [
            "Takto vypadají šifry a navigace na stanoviště. Mohou obsahovat libovolný text, třeba popis cesty:",
            "Jděte do kopce po silnici, dokud nenarazíte na velký dub."
        ],
        "prompt": "Jaký se zde nachází strom?",
        "submit_button": "Odeslat",
        "solutions": [
            {"type": "alphanumeric", "solution": "dub"},
            {"type": "alphanumeric", "solution": "velký dub"}
        ],
        "visible": "intro.visible",
        "on_solution_correct": "Správně!",
        "on_solution_incorrect": "Vaše odpověď je špatně. Přečtěte si text."
    }'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', 'd2cbdb77-01bf-4c1a-80e1-0f9895de08f1', 'start', 20, '{
        "type": "prompt",
        "name": "První šifra",
        "details": [
            "Takto vypadá šifra s nápovědami. Pro nápovědy se dají nastavit flexibilní časové limity.",
            "Zde se první dvě nápovědy odemknou chvíli po příchodu na šifru, třetí až po vyčerpání prvních dvou.",
            {"content": "Jste tu už více než třicet sekund.", "condition": "this.visible + 30 s"}
        ],
        "prompt": "Řešení:",
        "submit_button": "Odeslat",
        "solutions": [
            {"type": "number", "solution": 2},
            {"type": "alphanumeric", "solution": "dva"},
            {"type": "alphanumeric", "solution": "dve"}
        ],
        "visible": "to-start.solved",
        "on_solution_correct": "Správně!",
        "on_solution_incorrect": "Vaše odpověď je špatně. Použijte nápovědy.",
        "hints": [
            {
                "ident": "first",
                "name": "První nápověda",
                "content": ["Řešením je prvočíslo."],
                "available": "this.visible + 10 s",
                "visible": "always",
                "take_button": "Odemknout",
                "on_hint_taken": "Nápověda odemčena."
            },
            {
                "ident": "second",
                "name": "Druhá nápověda",
                "content": ["Řešením je sudé číslo."],
                "available": "this.visible + 20 s",
                "visible": "always",
                "take_button": "Odemknout",
                "on_hint_taken": "Nápověda odemčena."
            },
            {
                "ident": "third",
                "name": "Třetí nápověda",
                "content": ["Řešením je číslo 2."],
                "available": "(this.hint.first.taken & this.hint.second.taken) + 10 s",
                "visible": "always",
                "take_button": "Odemknout",
                "on_hint_taken": "Nápověda odemčena."
            }
        ]
    }'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '4e989707-f7e8-4c69-b189-95d9472036a1', 'answers', 30, '{
        "type": "prompt",
        "name": "Druhá šifra",
        "details": [
            "Šifry mohou mít více správných řešení. Při kontrole řešení se navíc ignoruje velikost písmen, diakritika, interpunkce a bílé znaky."
        ],
        "prompt": "Co se ignoruje při kontrole řešení?",
        "submit_button": "Odeslat",
        "solutions": [
            {"type": "alphanumeric", "solution": "velikost písmen"},
            {"type": "alphanumeric", "solution": "diakritika"},
            {"type": "alphanumeric", "solution": "interpunkce"},
            {"type": "alphanumeric", "solution": "bílé znaky"}
        ],
        "visible": "start.solved",
        "on_solution_correct": "Správně!",
        "on_solution_incorrect": "Špatné řešení. Přečtěte si text."
    }'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', 'd72c008b-02fa-4a60-8b8b-9663d3ec124a', 'bonus', 40, '{
        "type": "prompt",
        "name": "Bonus",
        "details": [
            "Je také možné řešit více šifer zároveň. Tato šifra je navíc bonusová, takže jí není nutné vyřešit k postupu.",
            "Nápověda k tomuto bonusu je nastavená tak, aby se odemkla po dokončení šifrovačky.",
            "Po dokonční šifrovačky se bonus za půl minuty uzamkne"
        ],
        "prompt": "Řešení:",
        "submit_button": "Odeslat",
        "solutions": [
            {"type": "alphanumeric", "solution": "mlha"}
        ],
        "visible": "start.solved",
        "disabled": "outro.visible + 30s",
        "on_solution_correct": "Správně!",
        "on_solution_incorrect": "Špatné řešení.",
        "hints": [
            {
                "ident": "main",
                "name": "Nápověda",
                "content": ["Řešením je \"mlha\"."],
                "available": "answers.solved",
                "visible": "always",
                "take_button": "Odemknout",
                "on_hint_taken": "Nápověda odemčena."
            }
        ]
    }'),
    ('f898f0d2-fb72-4046-a20b-7347a061b6a4', '97e528e5-09af-444a-a886-47a1b4183fb7', 'outro', 50, '{
        "type": "text",
        "heading": "Závěr",
        "content": [
            "Toto je konec této ukázky.",
            "Mezi funkce neobsažené v této ukázce patří například odpočty a možnost upravování parametrů hry pro různé týmy."
        ],
        "visible": "answers.solved"
    }');
