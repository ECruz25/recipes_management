-- This file should undo anything in `up.sql`
ALTER TABLE schedules
    DROP COLUMN amount;

