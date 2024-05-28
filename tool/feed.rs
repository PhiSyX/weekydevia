use std::{
    io::{BufReader, Write},
    path,
};

pub fn generate_rss(release_file: impl AsRef<path::Path>) -> crate::Result<()> {
    let release_name = release_file.as_ref().file_name().unwrap().to_string_lossy();
    // ex: 20240529_20240602
    assert!(release_name.len() == 17);
    let release_date = format!(
        "{}.{}.{} - {}.{}.{}",
        &release_name[0..=3],
        &release_name[4..=5],
        &release_name[6..=7],
        &release_name[9..=12],
        &release_name[13..=14],
        &release_name[14..=16],
    );

    let current_feed = std::fs::File::open("release/feed.xml")?;
    let mut channel = rss::Channel::read_from(BufReader::new(current_feed))?;

    let release_file = release_file.as_ref().join("README.md");
    let rss_content = std::fs::read_to_string(release_file)?;
    let rss_pub_date = chrono::Utc::now();
    let rss_title = format!("weekydevia: {}", &release_date);
    let rss_link = format!(
        "https://github.com/PhiSyX/weekydevia/blob/main/release/{}/README.md",
        &release_date,
    );
    let rss_description = format!("Resources of the week ({})", release_date);
    let rss_author = String::from("Mike 'PhiSyX' S.");

    channel.last_build_date.replace(rss_pub_date.to_string());

    channel.items.push(rss::Item {
        title: Some(rss_title),
        link: Some(rss_link),
        description: Some(rss_description),
        author: Some(rss_author),
        guid: Some(rss::Guid::default()),
        pub_date: Some(rss_pub_date.to_string()),
        content: Some(markdown2html::markdown(&rss_content)),
        ..Default::default()
    });

    let mut feed_file = std::fs::File::create("release/feed.xml")?;
    feed_file.write_all(channel.to_string().as_bytes())?;

    Ok(())
}
