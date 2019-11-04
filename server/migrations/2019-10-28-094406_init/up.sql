-- An event organised by a single user or group of users.
CREATE TABLE events (
    -- Internal system ID.
    event_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    -- The original markdown for the event description.
    about_md VARCHAR NOT NULL,
    -- The html for the event description.
    about_html VARCHAR NOT NULL,
    -- The street address.
    address VARCHAR NOT NULL,
    -- Whether an event has been approved for display.
    approved BOOLEAN DEFAULT FALSE NOT NULL,
    -- The city
    city VARCHAR NOT NULL,
    -- The country
    country VARCHAR NOT NULL,
    -- When this event was created on timetill.rs.
    created timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    -- The display name of the event.
    event VARCHAR NOT NULL,
    -- The start date of the event.
    event_date timestamptz NOT NULL,
    -- The event's duration amount.
    event_duration_amount SMALLINT NOT NULL CHECK (event_duration_step IS NOT NULL),
    -- The event's duration step.
    event_duration_step VARCHAR NOT NULL CHECK (event_duration_amount IS NOT NULL),
    -- The coordinates to the event's venue. If NULL, disables map integration.
    gps point,
    -- How often the event happens, can be NULL for once off events.
    occurance_amount SMALLINT CHECK (occurance_step IS NOT NULL),
    occurance_step VARCHAR CHECK (occurance_amount IS NOT NULL),
    -- The region.
    region VARCHAR NOT NULL,
    -- The URL safe slug for the event.
    slug VARCHAR UNIQUE NOT NULL
);

-- A GitHub user who has registered on timetill.rs
CREATE TABLE users (
    -- Github's ID for a user.
    github_id INT NOT NULL PRIMARY KEY,
    -- Last known GitHub name.
    github_name VARCHAR NOT NULL,
    -- Whether a user is allowed to review events for approval.
    reviewer BOOLEAN NOT NULL DEFAULT FALSE,
    -- Whether a user is allowed to post events without approval.
    trusted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE event_organisers (
    -- Foreign key table for organisers of events. Users in this table can edit
    -- and delete their events.

    organiser_id INT NOT NULL REFERENCES users (github_id) ON UPDATE CASCADE,
    event_id INT NOT NULL REFERENCES events (event_id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT event_organisers_id PRIMARY KEY (organiser_id, event_id)
);

-- Foreign key table for attendees of an event.
CREATE TABLE event_attendees (
    attendee_id INT NOT NULL REFERENCES users (github_id) ON UPDATE CASCADE,
    event_id INT NOT NULL REFERENCES events (event_id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- The date of when the attendee plans to attend the event. If they're
    -- a recurring attendee this is also the first event they attended.

    event_date timestamptz NOT NULL,
    -- Whether a attendee is subscribed to come regularly to the event.
    recurring_attendee BOOLEAN DEFAULT FALSE NOT NULL,
    CONSTRAINT event_attendees_id PRIMARY KEY (attendee_id, event_id)
);

