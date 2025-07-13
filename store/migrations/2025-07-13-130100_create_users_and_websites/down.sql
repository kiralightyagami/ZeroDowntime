-- This file should undo anything in `up.sql`

-- Remove foreign key constraints first
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_website_id_fkey";
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_region_id_fkey";

-- Drop tables in reverse order of creation
DROP TABLE "website_tick";
DROP TABLE "region";
DROP TABLE "website";

-- Drop the enum type
DROP TYPE "website_status";