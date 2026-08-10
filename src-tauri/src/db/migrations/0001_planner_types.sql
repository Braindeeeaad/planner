-- 1. Locations (GPS & Addresses)
CREATE TABLE locations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                -- e.g., "Home", "Campus Library"
    address TEXT NOT NULL,             -- Formatted street address
    latitude REAL NOT NULL,
    longitude REAL NOT NULL
);

-- 2. Event Contexts (Classes, Work, or Fixed Commitments)
CREATE TABLE event_contexts (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,               -- e.g., "Calculus I Lecture"
    category TEXT NOT NULL,            -- e.g., "class", "work", "appointment"
    start_location_id TEXT,
    end_location_id TEXT,
    recurring_rule TEXT,               -- e.g., "FREQ=WEEKLY;BYDAY=MO,WE,FR"
    FOREIGN KEY (start_location_id) REFERENCES locations(id),
    FOREIGN KEY (end_location_id) REFERENCES locations(id)
);

-- 3. Goals (Top-level aspirations)
CREATE TABLE goals (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,               -- e.g., "Pass Calculus with an A"
    target_date TEXT NOT NULL,         -- ISO8601 date string
    status TEXT NOT NULL               -- "active", "completed", "archived"
);

-- 4. Tasks (Actionable items, commutes, and checkpoints)
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    goal_id TEXT,
    event_context_id TEXT,
    title TEXT NOT NULL,
    task_type TEXT NOT NULL,           -- "standard", "commute", "study", "exam"
    base_duration INTEGER NOT NULL,    -- Estimated minutes
    scheduled_start TEXT,              -- ISO8601 timestamp
    scheduled_end TEXT,                -- ISO8601 timestamp
    urgency_score REAL DEFAULT 0.0,
    importance_score REAL DEFAULT 0.0,
    priority_weight REAL DEFAULT 0.0,
    status TEXT NOT NULL,              -- "pending", "completed", "postponed"
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE,
    FOREIGN KEY (event_context_id) REFERENCES event_contexts(id) ON DELETE SET NULL
);

-- 5. Task Dependencies (DAG Edges)
CREATE TABLE task_dependencies (
    predecessor_id TEXT NOT NULL,      -- Must finish first
    successor_id TEXT NOT NULL,        -- Depends on predecessor
    goal_id TEXT NOT NULL, 
    PRIMARY KEY (predecessor_id, successor_id),
    FOREIGN KEY (predecessor_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (successor_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (goal_id) REFERENCES goals(id)
);

-- 6. Habits (Templates that generate recurring tasks)
CREATE TABLE habits (
    id TEXT PRIMARY KEY,
    goal_id TEXT,
    title TEXT NOT NULL,
    frequency TEXT NOT NULL,           -- e.g., "daily", "weekly"
    target_time INTEGER NOT NULL,      -- Duration in minutes
    streak_count INTEGER DEFAULT 0,
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE SET NULL
);

-- 7. Task Feedback (Adaptive learning loop data)
CREATE TABLE task_feedback (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    estimated_duration INTEGER NOT NULL,
    actual_duration INTEGER NOT NULL,
    user_sentiment TEXT,               -- "easy", "just_right", "struggled", "overdue"
    completion_quality INTEGER,        -- Scale 1-5 evaluated via SLM
    created_at TEXT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- 8. User Ability Profile (Bayesian multipliers for time estimation)
CREATE TABLE user_ability_profiles (
    category TEXT PRIMARY KEY,         -- e.g., "coding", "writing", "math"
    velocity_multiplier REAL NOT NULL DEFAULT 1.0, -- Actual / Estimated ratio
    notes TEXT                         -- JSON or text summary updated by SLM coach
);

-- 9. Gradebook (Academic performance tracking)
CREATE TABLE grade_items (
    id TEXT PRIMARY KEY,
    event_context_id TEXT NOT NULL,    -- Links to the specific class
    title TEXT NOT NULL,               -- e.g., "Midterm Exam"
    weight REAL NOT NULL,              -- Percentage weight (e.g., 0.25 for 25%)
    earned_score REAL,                 -- Points earned
    max_score REAL NOT NULL,           -- Total possible points
    target_grade REAL,                 -- Desired score
    FOREIGN KEY (event_context_id) REFERENCES event_contexts(id) ON DELETE CASCADE
);