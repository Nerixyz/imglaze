alter table overlays
    alter column last_image
        type varchar(255) using last_image::varchar(255);