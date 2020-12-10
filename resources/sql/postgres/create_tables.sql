-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler  version: 0.9.3-beta1
-- PostgreSQL version: 13.0
-- Project Site: pgmodeler.io
-- Model Author: ---

-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: p0nygramm | type: DATABASE --
-- DROP DATABASE IF EXISTS p0nygramm;
--CREATE DATABASE p0nygramm;
-- ddl-end --


-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
	user_id serial NOT NULL,
	user_name varchar(40) NOT NULL,
	user_pass varchar(128) NOT NULL,
	user_is_mod bool NOT NULL DEFAULT false,
	CONSTRAINT users_pk PRIMARY KEY (user_id),
	CONSTRAINT user_name_unique UNIQUE (user_name)

);
-- ddl-end --
COMMENT ON COLUMN public.users.user_pass IS E'Hashed password';
-- ddl-end --

-- object: public."UploadType" | type: TYPE --
-- DROP TYPE IF EXISTS public."UploadType" CASCADE;
CREATE TYPE public."UploadType" AS
 ENUM ('Image','AnimatedImage','Video');
-- ddl-end --

-- object: public.comments | type: TABLE --
-- DROP TABLE IF EXISTS public.comments CASCADE;
CREATE TABLE public.comments (
	comment_id serial NOT NULL,
	comment_timestamp timestamp with time zone NOT NULL DEFAULT Now(),
	comment_text text NOT NULL,
	comment_poster integer NOT NULL,
	comment_upvotes integer NOT NULL DEFAULT 0,
	comment_upload integer NOT NULL,
	CONSTRAINT comments_pk PRIMARY KEY (comment_id)

);
-- ddl-end --

-- object: public.tags | type: TABLE --
-- DROP TABLE IF EXISTS public.tags CASCADE;
CREATE TABLE public.tags (
	tag_id serial NOT NULL,
	tag_text varchar(70) NOT NULL,
	CONSTRAINT taggs_pk PRIMARY KEY (tag_id),
	CONSTRAINT tag_text_unique UNIQUE (tag_text)

);
-- ddl-end --

-- object: public.tag_upload_map | type: TABLE --
-- DROP TABLE IF EXISTS public.tag_upload_map CASCADE;
CREATE TABLE public.tag_upload_map (
	tum_id serial NOT NULL,
	tag_upvotes integer NOT NULL DEFAULT 0,
	tag_poster integer NOT NULL,
	tag_id integer NOT NULL,
	upload_id integer NOT NULL,
	CONSTRAINT tag_upload_map_pk PRIMARY KEY (tum_id)

);
-- ddl-end --

-- object: public.user_banns | type: TABLE --
-- DROP TABLE IF EXISTS public.user_banns CASCADE;
CREATE TABLE public.user_banns (
	ban_id serial NOT NULL,
	ban_reason text NOT NULL DEFAULT 'Willkür',
	ban_start timestamp with time zone NOT NULL DEFAULT Now(),
	ban_duration integer NOT NULL DEFAULT 24,
	ban_user integer NOT NULL,
	CONSTRAINT user_banns_pk PRIMARY KEY (ban_id)

);
-- ddl-end --
COMMENT ON COLUMN public.user_banns.ban_duration IS E'Ban duration in hours';
-- ddl-end --

-- object: public.project_kvconfig | type: TABLE --
-- DROP TABLE IF EXISTS public.project_kvconfig CASCADE;
CREATE TABLE public.project_kvconfig (
	kv_key varchar(64) NOT NULL,
	kv_value_str varchar(64),
	kv_value_int integer,
	CONSTRAINT project_kvconfig_pk PRIMARY KEY (kv_key)

);
-- ddl-end --

INSERT INTO public.project_kvconfig (kv_key) VALUES (E'schema_version');
-- ddl-end --

-- object: public.votes_tum | type: TABLE --
-- DROP TABLE IF EXISTS public.votes_tum CASCADE;
CREATE TABLE public.votes_tum (
	vote_id serial NOT NULL,
	vote_tagmap integer NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	CONSTRAINT votes_tum_pk PRIMARY KEY (vote_id)

);
-- ddl-end --

-- object: public.votes_uploads | type: TABLE --
-- DROP TABLE IF EXISTS public.votes_uploads CASCADE;
CREATE TABLE public.votes_uploads (
	vote_id serial NOT NULL,
	vote_upload integer NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	CONSTRAINT votes_uploads_pk PRIMARY KEY (vote_id)

);
-- ddl-end --

-- object: public.votes_comments | type: TABLE --
-- DROP TABLE IF EXISTS public.votes_comments CASCADE;
CREATE TABLE public.votes_comments (
	vote_id serial NOT NULL,
	vote_comment integer NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	CONSTRAINT votes_comments_pk PRIMARY KEY (vote_id)

);
-- ddl-end --

-- object: username_uq | type: INDEX --
-- DROP INDEX IF EXISTS public.username_uq CASCADE;
CREATE UNIQUE INDEX username_uq ON public.users
	USING btree
	(
	  (LOWER(user_name))
	);
-- ddl-end --

-- object: public.uploads | type: TABLE --
-- DROP TABLE IF EXISTS public.uploads CASCADE;
CREATE TABLE public.uploads (
	upload_id serial NOT NULL,
	upload_filename varchar(70) NOT NULL,
	upload_timestamp timestamp with time zone NOT NULL DEFAULT Now(),
	upload_is_sfw bool NOT NULL DEFAULT true,
	upload_is_nsfw bool NOT NULL DEFAULT false,
	upload_type public."UploadType" NOT NULL DEFAULT Image,
	upload_upvotes integer NOT NULL DEFAULT 0,
	uploader integer NOT NULL,
	CONSTRAINT uploads_pk PRIMARY KEY (upload_id),
	CONSTRAINT upload_filename_unique UNIQUE (upload_filename)

);
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.comments DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.comments ADD CONSTRAINT user_fk FOREIGN KEY (comment_poster)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: upload_fk | type: CONSTRAINT --
-- ALTER TABLE public.comments DROP CONSTRAINT IF EXISTS upload_fk CASCADE;
ALTER TABLE public.comments ADD CONSTRAINT upload_fk FOREIGN KEY (comment_upload)
REFERENCES public.uploads (upload_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: tag_poster_fk | type: CONSTRAINT --
-- ALTER TABLE public.tag_upload_map DROP CONSTRAINT IF EXISTS tag_poster_fk CASCADE;
ALTER TABLE public.tag_upload_map ADD CONSTRAINT tag_poster_fk FOREIGN KEY (tag_poster)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: tag_link | type: CONSTRAINT --
-- ALTER TABLE public.tag_upload_map DROP CONSTRAINT IF EXISTS tag_link CASCADE;
ALTER TABLE public.tag_upload_map ADD CONSTRAINT tag_link FOREIGN KEY (tag_id)
REFERENCES public.tags (tag_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: upload_link | type: CONSTRAINT --
-- ALTER TABLE public.tag_upload_map DROP CONSTRAINT IF EXISTS upload_link CASCADE;
ALTER TABLE public.tag_upload_map ADD CONSTRAINT upload_link FOREIGN KEY (upload_id)
REFERENCES public.uploads (upload_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.user_banns DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.user_banns ADD CONSTRAINT user_fk FOREIGN KEY (ban_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: tum_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_tum DROP CONSTRAINT IF EXISTS tum_fk CASCADE;
ALTER TABLE public.votes_tum ADD CONSTRAINT tum_fk FOREIGN KEY (vote_tagmap)
REFERENCES public.tag_upload_map (tum_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_tum DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.votes_tum ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: upload_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_uploads DROP CONSTRAINT IF EXISTS upload_fk CASCADE;
ALTER TABLE public.votes_uploads ADD CONSTRAINT upload_fk FOREIGN KEY (vote_upload)
REFERENCES public.uploads (upload_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_uploads DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.votes_uploads ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: comment_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_comments DROP CONSTRAINT IF EXISTS comment_fk CASCADE;
ALTER TABLE public.votes_comments ADD CONSTRAINT comment_fk FOREIGN KEY (vote_comment)
REFERENCES public.comments (comment_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_comments DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.votes_comments ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: uploader_fk | type: CONSTRAINT --
-- ALTER TABLE public.uploads DROP CONSTRAINT IF EXISTS uploader_fk CASCADE;
ALTER TABLE public.uploads ADD CONSTRAINT uploader_fk FOREIGN KEY (uploader)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


