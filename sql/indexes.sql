CREATE INDEX idx_sections_resume
ON resume_sections(resume_id);

CREATE INDEX idx_projects_section
ON projects(section_id);

CREATE INDEX idx_experience_section
ON work_experiences(section_id);