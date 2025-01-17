CREATE TABLE IF NOT EXISTS granths (
    id UUID PRIMARY KEY,
    name varchar(100) NOT NULL,
    size integer,
    distributed boolean DEFAULT false,
    ext varchar(20) NOT NULL,
    total_segments INT DEFAULT 0,
    ingress_node_id UUID REFERENCES nodes(id)
);