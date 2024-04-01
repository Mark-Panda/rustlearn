
CREATE TABLE "public"."openais" (
     "id" uuid DEFAULT uuid_generate_v4 (),
     "chat_id" varchar(64) NOT NULL COLLATE "pg_catalog"."default",
     "message"  text NOT NULL,
     "created_at" timestamptz(6) NOT NULL DEFAULT now(),
     "updated_at" timestamptz(6) NOT NULL DEFAULT now(),
     "deleted_at" timestamptz(6),
     CONSTRAINT "openais_pkey" PRIMARY KEY ("id")
);

COMMENT ON COLUMN "public"."openais"."id" IS '主键ID';

COMMENT ON COLUMN "public"."openais"."chat_id" IS '关联的聊天ID';

COMMENT ON COLUMN "public"."openais"."message" IS '问题内容';

COMMENT ON COLUMN "public"."openais"."created_at" IS '创建时间';

COMMENT ON COLUMN "public"."openais"."updated_at" IS '更新时间';

COMMENT ON COLUMN "public"."openais"."deleted_at" IS '删除时间';