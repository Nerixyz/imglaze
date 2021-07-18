drop index overlays_created_by_for_user_uindex;

-- restore previous index
create unique index overlays_created_by_for_user_uindex
    on overlays (created_by, for_user);