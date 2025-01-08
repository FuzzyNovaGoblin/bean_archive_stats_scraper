use std::collections::HashSet;

use scraper::Selector;

pub async fn build_notification_msg(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parser = scraper::Html::parse_fragment(&data);

    let outer_selector =
        Selector::parse("ul.statistics.index.group li.fandom.listbox.group ul.index.group li")?;

    let name_selector = Selector::parse("dl dt a")?;
    let stat_name_selector = Selector::parse("dl.stats dt")?;
    let stat_selector = Selector::parse("dl.stats dd")?;

    let mut stat_set = HashSet::new();

    for stat_block in parser.select(&outer_selector) {
        let mut working_string = String::new();

        working_string.push_str(&format!(
            "- {}:\n",
            stat_block
                .select(&name_selector)
                .next()
                .unwrap()
                .inner_html()
        ));
        for (stat_name, stat) in stat_block
            .select(&stat_name_selector)
            .zip(stat_block.select(&stat_selector))
        {
            working_string.push_str(&format!(
                "\t{n} {s}\n",
                n = stat_name.inner_html(),
                s = stat.inner_html(),
            ));
        }

        // println!("{}", working_string);
        stat_set.insert(working_string);
    }

    let mut stats = stat_set.into_iter().collect::<Vec<_>>();
    stats.sort();
    Ok(stats.join("\n"))
}
