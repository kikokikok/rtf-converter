-- Description: Table for storing files with associated metadata
CREATE TABLE files (
                       id UUID DEFAULT uuid_generate_v4 (),                             -- UUID identifier for each file
                       tenant_id UUID,                      -- UUID of the associated tenant
                       owner_id UUID NULL,                  -- UUID of the file owner (optional)
                       file_binary_content BYTEA,           -- Binary data representing the file content
                       content_type VARCHAR(255),           -- Content type of the file
                       file_name VARCHAR(255),              -- Name of the file
                       file_size BIGINT,                    -- Size of the file in bytes
                       insertion_date TIMESTAMP,            -- Timestamp when the file was inserted
                       max_age INTERVAL DEFAULT '1 month',   -- Maximum age of the file (default is 1 month)
                       templating_engine VARCHAR(255),      -- Name of the templating engine used
                       templating_engine_version VARCHAR(50),-- Version of the templating engine
                       version INTEGER,                     -- Version number of the file
                       PRIMARY KEY (id, tenant_id, version)
);-- Add up migration script here
