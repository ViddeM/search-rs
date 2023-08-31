CREATE TABLE indexed_website(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT,
    domain TEXT NOT NULL,
    indexed_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (domain)
);

CREATE INDEX ON indexed_website (lower(title)); 

CREATE TABLE website_link(
    parent_website UUID NOT NULL,
    child_website UUID NOT NULL,

    PRIMARY KEY (parent_website, child_website),

    FOREIGN KEY (parent_website) REFERENCES indexed_website(id),
    FOREIGN KEY (child_website) REFERENCES indexed_website(id)
);

CREATE TABLE website_to_index(
    domain TEXT PRIMARY KEY,
    added_at TIMESTAMPTZ NOT NULL DEFAULT now()
);