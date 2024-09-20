--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3
-- Dumped by pg_dump version 16.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: key_kind; Type: TYPE; Schema: public; Owner: casper
--

CREATE TYPE public.key_kind AS ENUM (
    'api',
    'token'
);


ALTER TYPE public.key_kind OWNER TO casper;

--
-- Name: set_updated_at(); Type: FUNCTION; Schema: public; Owner: casper
--

CREATE FUNCTION public.set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$ begin
    new.updated_at = now();
    return new;
end; $$;


ALTER FUNCTION public.set_updated_at() OWNER TO casper;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO casper;

--
-- Name: definitions; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public.definitions (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name text NOT NULL,
    value jsonb NOT NULL,
    is_default boolean DEFAULT false NOT NULL,
    service_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.definitions OWNER TO casper;

--
-- Name: identifiers; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public.identifiers (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    value text NOT NULL,
    user_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.identifiers OWNER TO casper;

--
-- Name: keys; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public.keys (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    kind public.key_kind NOT NULL,
    value bytea NOT NULL,
    priority integer NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    service_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.keys OWNER TO casper;

--
-- Name: services; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public.services (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name text NOT NULL,
    redirect_url text NOT NULL,
    logout_url text NOT NULL,
    webhook_url text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.services OWNER TO casper;

--
-- Name: users; Type: TABLE; Schema: public; Owner: casper
--

CREATE TABLE public.users (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    service_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.users OWNER TO casper;

--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: definitions definitions_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.definitions
    ADD CONSTRAINT definitions_pkey PRIMARY KEY (id);


--
-- Name: definitions definitions_service_id_is_default_key; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.definitions
    ADD CONSTRAINT definitions_service_id_is_default_key UNIQUE (service_id, is_default);


--
-- Name: identifiers identifiers_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.identifiers
    ADD CONSTRAINT identifiers_pkey PRIMARY KEY (id);


--
-- Name: identifiers identifiers_value_key; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.identifiers
    ADD CONSTRAINT identifiers_value_key UNIQUE (value);


--
-- Name: keys keys_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.keys
    ADD CONSTRAINT keys_pkey PRIMARY KEY (id);


--
-- Name: keys keys_service_id_kind_priority_key; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.keys
    ADD CONSTRAINT keys_service_id_kind_priority_key UNIQUE (service_id, kind, priority);


--
-- Name: services services_name_key; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_name_key UNIQUE (name);


--
-- Name: services services_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: definitions set_updated_at; Type: TRIGGER; Schema: public; Owner: casper
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.definitions FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: identifiers set_updated_at; Type: TRIGGER; Schema: public; Owner: casper
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.identifiers FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: keys set_updated_at; Type: TRIGGER; Schema: public; Owner: casper
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.keys FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: services set_updated_at; Type: TRIGGER; Schema: public; Owner: casper
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.services FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: users set_updated_at; Type: TRIGGER; Schema: public; Owner: casper
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.users FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: definitions definitions_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.definitions
    ADD CONSTRAINT definitions_service_id_fkey FOREIGN KEY (service_id) REFERENCES public.services(id) ON DELETE CASCADE;


--
-- Name: identifiers identifiers_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.identifiers
    ADD CONSTRAINT identifiers_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: keys keys_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.keys
    ADD CONSTRAINT keys_service_id_fkey FOREIGN KEY (service_id) REFERENCES public.services(id) ON DELETE CASCADE;


--
-- Name: users users_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: casper
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_service_id_fkey FOREIGN KEY (service_id) REFERENCES public.services(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

