-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler  version: 0.9.2
-- PostgreSQL version: 12.0
-- Project Site: pgmodeler.io
-- Model Author: ---


-- Database creation must be done outside a multicommand file.
-- These commands were put in this file only as a convenience.
-- -- object: p0nygramm | type: DATABASE --
-- -- DROP DATABASE IF EXISTS p0nygramm;
-- CREATE DATABASE p0nygramm
-- 	OWNER = postgres;
-- -- ddl-end --
-- 

-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
	user_id serial NOT NULL,
	user_name varchar(32) NOT NULL,
	user_pass varchar(64) NOT NULL,
	CONSTRAINT users_pk PRIMARY KEY (user_id)

);
-- ddl-end --
COMMENT ON COLUMN public.users.user_pass IS E'Hashed password';
-- ddl-end --
-- ALTER TABLE public.users OWNER TO postgres;
-- ddl-end --

-- object: public.uploads | type: TABLE --
-- DROP TABLE IF EXISTS public.uploads CASCADE;
CREATE TABLE public.uploads (
	upload_id serial NOT NULL,
	upload_filename varchar(64) NOT NULL,
	upload_timestamp timestamp with time zone NOT NULL DEFAULT Now(),
	upload_upvotes integer NOT NULL DEFAULT 0,
	uploader integer NOT NULL,
	CONSTRAINT uploads_pk PRIMARY KEY (upload_id)

);
-- ddl-end --
COMMENT ON COLUMN public.uploads.upload_upvotes IS E'To display the votes fastly';
-- ddl-end --
-- ALTER TABLE public.uploads OWNER TO postgres;
-- ddl-end --

-- object: public.comments | type: TABLE --
-- DROP TABLE IF EXISTS public.comments CASCADE;
CREATE TABLE public.comments (
	comment_id serial NOT NULL,
	comment_timestamp timestamp with time zone NOT NULL DEFAULT Now(),
	comment_upvotes integer NOT NULL DEFAULT 0,
	comment_poster integer NOT NULL,
	comment_upload integer NOT NULL,
	CONSTRAINT comments_pk PRIMARY KEY (comment_id)

);
-- ddl-end --
-- ALTER TABLE public.comments OWNER TO postgres;
-- ddl-end --

-- object: public.votes_uploads | type: TABLE --
-- DROP TABLE IF EXISTS public.votes_uploads CASCADE;
CREATE TABLE public.votes_uploads (
	vote_id serial NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	vote_upload integer NOT NULL,
	CONSTRAINT votes_uploads_pk PRIMARY KEY (vote_id)

);
-- ddl-end --
-- ALTER TABLE public.votes_uploads OWNER TO postgres;
-- ddl-end --

-- object: public.votes_comments | type: TABLE --
-- DROP TABLE IF EXISTS public.votes_comments CASCADE;
CREATE TABLE public.votes_comments (
	vote_id serial NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	vote_comment integer NOT NULL,
	CONSTRAINT votes_comments_pk PRIMARY KEY (vote_id)

);
-- ddl-end --
-- ALTER TABLE public.votes_comments OWNER TO postgres;
-- ddl-end --

-- object: public.tags | type: TABLE --
-- DROP TABLE IF EXISTS public.tags CASCADE;
CREATE TABLE public.tags (
	tag_id serial NOT NULL,
	tag_text varchar(64) NOT NULL,
	tag_poster integer NOT NULL,
	CONSTRAINT taggs_pk PRIMARY KEY (tag_id)

);
-- ddl-end --
-- ALTER TABLE public.tags OWNER TO postgres;
-- ddl-end --

-- object: public.tag_upload_map | type: TABLE --
-- DROP TABLE IF EXISTS public.tag_upload_map CASCADE;
CREATE TABLE public.tag_upload_map (
	tum_id serial NOT NULL,
	tag_upvotes integer NOT NULL DEFAULT 0,
	tag_id integer NOT NULL,
	upload_id integer NOT NULL,
	CONSTRAINT tag_upload_map_pk PRIMARY KEY (tum_id)

);
-- ddl-end --
-- ALTER TABLE public.tag_upload_map OWNER TO postgres;
-- ddl-end --

-- object: public.tag_votes | type: TABLE --
-- DROP TABLE IF EXISTS public.tag_votes CASCADE;
CREATE TABLE public.tag_votes (
	vote_id serial NOT NULL,
	vote_number integer NOT NULL DEFAULT 0,
	vote_user integer NOT NULL,
	vote_tagmap integer NOT NULL,
	CONSTRAINT tag_votes_pk PRIMARY KEY (vote_id)

);
-- ddl-end --
-- ALTER TABLE public.tag_votes OWNER TO postgres;
-- ddl-end --

-- object: public.user_banns | type: TABLE --
-- DROP TABLE IF EXISTS public.user_banns CASCADE;
CREATE TABLE public.user_banns (
	ban_id serial NOT NULL,
	ban_reason varchar(64) NOT NULL DEFAULT 'Willkür',
	ban_start timestamp with time zone NOT NULL DEFAULT Now(),
	ban_duration integer NOT NULL DEFAULT 24,
	ban_user integer NOT NULL,
	CONSTRAINT user_banns_pk PRIMARY KEY (ban_id)

);
-- ddl-end --
COMMENT ON COLUMN public.user_banns.ban_duration IS E'Ban duration in hours';
-- ddl-end --
-- ALTER TABLE public.user_banns OWNER TO postgres;
-- ddl-end --

-- object: public.project_kvconfig | type: TABLE --
-- DROP TABLE IF EXISTS public.project_kvconfig CASCADE;
CREATE TABLE public.project_kvconfig (
	kv_key varchar(64) NOT NULL,
	kv_value varchar(64) NOT NULL,
	CONSTRAINT project_kvconfig_pk PRIMARY KEY (kv_key)

);
-- ddl-end --
-- ALTER TABLE public.project_kvconfig OWNER TO postgres;
-- ddl-end --

INSERT INTO public.project_kvconfig (kv_key, kv_value) VALUES (E'schema_version', E'1');
-- ddl-end --

-- object: uploader_fk | type: CONSTRAINT --
-- ALTER TABLE public.uploads DROP CONSTRAINT IF EXISTS uploader_fk CASCADE;
ALTER TABLE public.uploads ADD CONSTRAINT uploader_fk FOREIGN KEY (uploader)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
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

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_uploads DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.votes_uploads ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: upload_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_uploads DROP CONSTRAINT IF EXISTS upload_fk CASCADE;
ALTER TABLE public.votes_uploads ADD CONSTRAINT upload_fk FOREIGN KEY (vote_upload)
REFERENCES public.uploads (upload_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_comments DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.votes_comments ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: comment_fk | type: CONSTRAINT --
-- ALTER TABLE public.votes_comments DROP CONSTRAINT IF EXISTS comment_fk CASCADE;
ALTER TABLE public.votes_comments ADD CONSTRAINT comment_fk FOREIGN KEY (vote_comment)
REFERENCES public.comments (comment_id) MATCH FULL
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
-- ALTER TABLE public.tag_votes DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.tag_votes ADD CONSTRAINT user_fk FOREIGN KEY (vote_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: tagmap_fk | type: CONSTRAINT --
-- ALTER TABLE public.tag_votes DROP CONSTRAINT IF EXISTS tagmap_fk CASCADE;
ALTER TABLE public.tag_votes ADD CONSTRAINT tagmap_fk FOREIGN KEY (vote_tagmap)
REFERENCES public.tag_upload_map (tum_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.user_banns DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.user_banns ADD CONSTRAINT user_fk FOREIGN KEY (ban_user)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


