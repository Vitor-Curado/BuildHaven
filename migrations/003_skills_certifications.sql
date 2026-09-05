-- =====================================================
-- Migration 003: Skills & Certifications
-- =====================================================



-- =====================================================
-- TABLE: skills
-- =====================================================

CREATE TABLE skills (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL UNIQUE,

    category TEXT

);

-- =====================================================
-- TABLE: resume_skills
-- =====================================================

CREATE TABLE resume_skills (

    resume_id UUID NOT NULL
        REFERENCES resumes(id)
        ON DELETE CASCADE,

    skill_id UUID NOT NULL
        REFERENCES skills(id)
        ON DELETE CASCADE,

    display_order INTEGER NOT NULL,

    PRIMARY KEY (resume_id, skill_id)

);

-- =====================================================
-- TABLE: certifications
-- =====================================================

CREATE TABLE certifications (

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    section_id UUID NOT NULL
        REFERENCES resume_sections(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    issuer TEXT,

    credential_id TEXT,

    issue_date DATE,
    expiry_date DATE,

    show_issue_date BOOLEAN NOT NULL DEFAULT TRUE,
    show_expiry_date BOOLEAN NOT NULL DEFAULT TRUE,

    display_order INTEGER NOT NULL

);