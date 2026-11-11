BEGIN;

-- One comment per legacy failure reason. Comment ids sit in a reserved high
-- range so they cannot collide with Snowflake ids (about 1e18 today).
INSERT INTO comments (id, entity_type, entity_id, added_at, added_by, message)
SELECT
    8000000000000000000 + ROW_NUMBER() OVER (ORDER BY id),
    'panorama',
    id,
    created_at,
    NULL,
    CASE
        WHEN failure_reason LIKE 'Processing failed: %' THEN failure_reason
        ELSE 'Processing failed: ' || failure_reason
    END
FROM panoramas
WHERE failure_reason IS NOT NULL AND TRIM(failure_reason) <> '';

ALTER TABLE panoramas DROP COLUMN failure_reason;

COMMIT;
