-- =====================================================
-- Migration 002: Education & Work Experience
-- =====================================================



-- =====================================================
-- TABLE: education_entries
-- =====================================================

CREATE TABLE education_entries (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    section_id UUID NOT NULL
        REFERENCES resume_sections(id)
        ON DELETE CASCADE,

    institution TEXT NOT NULL,

    program TEXT,

    credential TEXT,

    location TEXT,

    start_date DATE,
    end_date DATE,

    show_start_date BOOLEAN NOT NULL DEFAULT TRUE,
    show_end_date BOOLEAN NOT NULL DEFAULT TRUE,

    display_order INTEGER NOT NULL

);

-- =====================================================
-- TABLE: work_experiences
-- =====================================================

CREATE TABLE work_experiences (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    section_id UUID NOT NULL
        REFERENCES resume_sections(id)
        ON DELETE CASCADE,

    company_name TEXT NOT NULL,

    job_title TEXT NOT NULL,

    location TEXT,

    start_date DATE,
    end_date DATE,

    show_start_date BOOLEAN NOT NULL DEFAULT TRUE,
    show_end_date BOOLEAN NOT NULL DEFAULT TRUE,

    is_current BOOLEAN NOT NULL DEFAULT FALSE,

    description TEXT,

    display_order INTEGER NOT NULL

);

-- =====================================================
-- TABLE: job_duties
-- =====================================================

CREATE TABLE job_duties (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    work_experience_id UUID NOT NULL
        REFERENCES work_experiences(id)
        ON DELETE CASCADE,

    description TEXT NOT NULL,

    display_order INTEGER NOT NULL

);