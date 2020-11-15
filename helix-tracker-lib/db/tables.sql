
CREATE TABLE tracker_v2.type
(
    id character varying(64) COLLATE pg_catalog."default" NOT NULL,
    name character varying(128) COLLATE pg_catalog."default" NOT NULL,
    expire_dafter character varying(256) COLLATE pg_catalog."default",
    refresh_every character varying(256) COLLATE pg_catalog."default",
    CONSTRAINT type_pkey PRIMARY KEY (id)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE tracker_v2.type
    OWNER to helix;


CREATE TABLE tracker_v2.item
(
    id uuid NOT NULL DEFAULT userstore.uuid_generate_v4(),
    configuration jsonb,
    expired_after character varying(256) COLLATE pg_catalog."default",
    refresh_every character varying(256) COLLATE pg_catalog."default",
    created_on timestamp(6) with time zone NOT NULL DEFAULT now(),
    updated_on timestamp(6) with time zone DEFAULT NULL::timestamp with time zone,
    owner_ uuid NOT NULL,
    type_ character varying(64) COLLATE pg_catalog."default",
    CONSTRAINT item_pkey PRIMARY KEY (id),
    CONSTRAINT item_type__fkey FOREIGN KEY (type_)
        REFERENCES tracker_v2.type (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE tracker_v2.item
    OWNER to helix;


CREATE TABLE tracker_v2.log
(
    uuid uuid NOT NULL DEFAULT userstore.uuid_generate_v4(),
    hash character varying(256) COLLATE pg_catalog."default",
    data jsonb,
    created_on timestamp(6) with time zone NOT NULL DEFAULT now(),
    item_ uuid NOT NULL,
    CONSTRAINT log_pkey PRIMARY KEY (uuid),
    CONSTRAINT log_item__fkey FOREIGN KEY (item_)
        REFERENCES tracker_v2.item (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE tracker_v2.log
    OWNER to helix;