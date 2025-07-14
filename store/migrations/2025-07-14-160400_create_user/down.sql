-- This file should undo anything in `up.sql`

-- DropForeignKey
ALTER TABLE "website" DROP CONSTRAINT "website_user_id_fkey";

-- DropTable
DROP TABLE "user";

-- AlterTable
ALTER TABLE "website_tick" DROP COLUMN "createdAt";

-- AlterTable
ALTER TABLE "website" DROP COLUMN "user_id";
