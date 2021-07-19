alter table overlays
    alter column last_image
        type varchar(350) using last_image::varchar(350);