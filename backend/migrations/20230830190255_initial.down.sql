-- Add down migration script here
CREATE TABLE website_index(
    id UUID PRIMARY KEY DEFAULT generate_random_uuid(),
    title TEXT NOT NULL,
    domain TEXT NOT NULL,
    indexed_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE website_link(
    parent_website UUID NOT NULL,
    child_website UUID NOT NULL,

    PRIMARY KEY (parent_website, child_website)
)