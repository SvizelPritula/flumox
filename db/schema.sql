CREATE TABLE public.game
(
    id uuid NOT NULL,
    name text NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE public.team
(
    game uuid NOT NULL,
    id uuid NOT NULL,
    name text NOT NULL,
    PRIMARY KEY (game, id),
    FOREIGN KEY (game)
        REFERENCES public.game (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE TABLE public.widget
(
    game uuid NOT NULL,
    id uuid NOT NULL,
    name uuid NOT NULL,
    config jsonb NOT NULL,
    PRIMARY KEY (game, id),
    FOREIGN KEY (game)
        REFERENCES public.game (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE TABLE public.state
(
    game uuid NOT NULL,
    team uuid NOT NULL,
    widget uuid NOT NULL,
    state jsonb NOT NULL,
    PRIMARY KEY (game, team, widget),
    FOREIGN KEY (game, team)
        REFERENCES public.team (game, id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (game, widget)
        REFERENCES public.widget (game, id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);
