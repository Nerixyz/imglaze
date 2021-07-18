drop index overlays_created_by_for_user_uindex;

create unique index overlays_created_by_for_user_uindex
    on overlays (for_user);