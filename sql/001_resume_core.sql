-- =====================================================
-- Migration 001: Resume Core Schema
-- =====================================================

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =====================================================
-- ENUM: Section Types
-- =====================================================

CREATE TYPE section_type_enum AS ENUM (
    'education',
    'projects',
    'experience',
    'skills',
    'certifications'
);

-- =====================================================
-- TABLE: resumes
-- =====================================================

CREATE TABLE resumes (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL,

    title TEXT NOT NULL,
    summary TEXT,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()

);

-- =====================================================
-- TABLE: resume_sections
-- =====================================================

CREATE TABLE resume_sections (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    resume_id UUID NOT NULL
        REFERENCES resumes(id)
        ON DELETE CASCADE,

    section_type section_type_enum NOT NULL,

    title TEXT NOT NULL,

    display_order INTEGER NOT NULL,

    is_visible BOOLEAN NOT NULL DEFAULT TRUE

);

-- =====================================================
-- TABLE: projects
-- =====================================================

CREATE TABLE projects (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    section_id UUID NOT NULL
        REFERENCES resume_sections(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    description TEXT,

    repository_url TEXT,
    live_url TEXT,

    start_date DATE,
    end_date DATE,

    show_start_date BOOLEAN NOT NULL DEFAULT TRUE,
    show_end_date BOOLEAN NOT NULL DEFAULT TRUE,

    is_ongoing BOOLEAN NOT NULL DEFAULT FALSE,

    display_order INTEGER NOT NULL

);

-- =====================================================
-- TABLE: project_highlights
-- =====================================================

CREATE TABLE project_highlights (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    project_id UUID NOT NULL
        REFERENCES projects(id)
        ON DELETE CASCADE,

    description TEXT NOT NULL,

    display_order INTEGER NOT NULL

);

-- =====================================================
-- TABLE: technologies
-- =====================================================

CREATE TABLE technologies (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL UNIQUE

);

-- =====================================================
-- TABLE: project_technologies
-- =====================================================

CREATE TABLE project_technologies (

    project_id UUID NOT NULL
        REFERENCES projects(id)
        ON DELETE CASCADE,

    technology_id UUID NOT NULL
        REFERENCES technologies(id)
        ON DELETE CASCADE,

    PRIMARY KEY (project_id, technology_id)

);