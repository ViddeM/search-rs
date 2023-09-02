CREATE TABLE DOMAIN (
    id          UUID NOT NULL DEFAULT gen_random_uuid(),
    domain      TEXT NOT NULL,
    indexed_at  TIMESTAMPTZ,
    added_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    PRIMARY KEY (id),
    
    UNIQUE (DOMAIN)
);

CREATE TABLE website_page(
    id          UUID NOT NULL DEFAULT gen_random_uuid(),
    domain      UUID NOT NULL,
    title       TEXT,
    page_url    TEXT NOT NULL,
    indexed_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id),

    FOREIGN KEY (DOMAIN) REFERENCES DOMAIN (id),

    UNIQUE (page_url)
);

CREATE INDEX ON website_page(LOWER(title));

CREATE TABLE domain_link(
    parent_domain   UUID NOT NULL,
    child_domain    UUID NOT NULL,

    PRIMARY KEY (parent_domain, child_domain),

    FOREIGN KEY (parent_domain) REFERENCES DOMAIN (id),
    FOREIGN KEY (child_domain) REFERENCES DOMAIN (id)
);
