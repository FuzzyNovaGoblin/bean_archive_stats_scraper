# Bean Archive Stats Scraper

This was not originally intended to be used by anyone else I just wanted to make it so my partner could more easily keep track of their stats on [archiveofourown](archiveofourown.org). Everything is configurable so anyone could use it for themselves. Many parts of this were taken from an [existing project of mine](https://github.com/FuzzyNovaGoblin/obsidian_notifications) so some elements of the code might be a tad clunky as they are striped down elements of a more complex project.

To run this create a config file at `/etc/bean_archive_stats_scraper/config.toml`. There is an example file included `EXAMPLE-config.toml` edit this and move it to the correct location. If you want to run this as a daemon so you can better track your stats over time, modify `EXAMPLE-bean_archive_stats_scraper.service` and move it to `/etc/systemd/system/bean_archive_stats_scraper.service` then enable and start it (example command: `systemctl enable --now bean_archive_stats_scraper.service`)

I still plan on and adding additional features to this project

## Planed Features

- [X] scape stats from ao3
- [X] eliminate duplicate entries that are listed in multiple categories
- [X] send stats to discord (in file so they don't get cut off from being too long)
- [X] sort entries so they are in a consistent order
- [X] enable timer funtionality and add systemd service so this runs as a daemon and scrapes every day at the same time
- [ ] include a way to login on its own and not require a new cookie to manually be entered every week
- [ ] add option to save stats locally and optionally not try to send them to discord
- [ ] add command to run scrape immediately
  - should only add the command to chanel listed in config, don't list the command in every server the bot is in
- [ ] if stats are going to exceed discord max file size send data as multiple files
  - I don't expect this to ever happen but if I make it for my [notifications](https://github.com/FuzzyNovaGoblin/obsidian_notifications) I should add it here too
- [ ] make file type sent configurable in toml (not sure how useful this actually would be)
