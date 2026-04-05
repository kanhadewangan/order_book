-- This file should undo anything in `up.sql`
IF EXISTS (SELECT 1 FROM pg_tables WHERE tablename = 'orders') THEN
    DROP TABLE orders;
END IF;

