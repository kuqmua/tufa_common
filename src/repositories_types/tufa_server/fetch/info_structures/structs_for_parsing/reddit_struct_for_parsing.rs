#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize, //Eq,
)]
pub struct RedditStructForParsing {
    pub data: RedditStructForParsingVector,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize, //Eq,
)]
pub struct RedditStructForParsingVector {
    pub children: Vec<RedditStructForParsingVectorChild>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize, //Eq,
)]
pub struct RedditStructForParsingVectorChild {
    pub data: RedditStructForParsingVectorChildData,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize, //Eq,
)]
pub struct RedditStructForParsingVectorChildData {
    // media_embed ???
    // user_reports ???
    // category ???
    // selftext_html ???
    // likes ??? null into what ???
    // banned_at_utc null into what ???
    // view_count ???
    // preview ?? //maybe but without auth or something i cannot reach image links
    // mod_reason_by ???
    // removal_reason ???
    // report_reasons ???
    // contest_mode ???
    // mod_reports ???
    // media ???
    //////////
    pub url_overridden_by_dest: Option<String>,
    pub link: Option<String>,
    pub subreddit: Option<String>,
    pub selftext: Option<String>,
    pub id: Option<String>,
    pub author: Option<String>,
    pub author_fullname: Option<String>,
    pub title: Option<String>,
    pub domain: Option<String>, //host site
    pub permalink: Option<String>,
    pub thumbnail: Option<String>, //image link todo
    pub url: Option<String>,
    pub subreddit_id: Option<String>,
    pub name: Option<String>,
    pub created_utc: Option<f64>,
    pub subreddit_subscribers: Option<f64>,
    pub ups: Option<f64>,
    pub score: Option<f64>, //difference between score and ups?
    pub num_comments: Option<u64>,
    pub downs: Option<f64>,
    pub upvote_ratio: Option<f64>,
    pub total_awards_received: Option<f64>,
    pub created: Option<f64>, //time
    pub over_18: Option<bool>,
    pub quarantine: Option<bool>,
    pub is_self: Option<bool>, //can be usefull in client app
    pub saved: Option<bool>,
    pub hidden: Option<bool>,
    pub is_original_content: Option<bool>,
    pub is_reddit_media_domain: Option<bool>,
    pub is_meta: Option<bool>,
    pub allow_live_comments: Option<bool>,
    pub archived: Option<bool>,
    pub is_crosspostable: Option<bool>,
    pub pinned: Option<bool>,
    pub media_only: Option<bool>,
    pub spoiler: Option<bool>,
    pub locked: Option<bool>,
    pub visited: Option<bool>,
    pub send_replies: Option<bool>,
    pub stickied: Option<bool>,
    pub is_video: Option<bool>,
}
// "approved_at_utc": null,//dont think so
// "subreddit": "3Dprinting",//already
// "selftext": "",//already
// "author_fullname": "t2_n6r6oak",//already
// "saved": false,//already
// "mod_reason_title": null,//dont know that it is
// "gilded": 0,//dont know that it is
// "clicked": false,//dont think so
// "title": "Experiment - load coordinates from *.stl (binary) to excel, it's crazy.",//already
// "link_flair_richtext": [//dont think so
// {
// "e": "text",
// "t": "Image"
// }
// ],
// "subreddit_name_prefixed": "r/3Dprinting",//dont think so
// "hidden": false,//already
// "pwls": 6,//dont know that it is
// "link_flair_css_class": "g",//dont think so
// "downs": 0,//already
// "thumbnail_height": 102,//dont think so
// "top_awarded_type": null,//dont know that it is
// "hide_score": true,//dont think so
// "name": "t3_n5anff",//already
// "quarantine": false,//already
// "link_flair_text_color": "dark",//dont think so
// "upvote_ratio": 1,//already
// "author_flair_background_color": null,//dont think so
// "subreddit_type": "public",//dont think so
// "ups": 1,//already
// "total_awards_received": 0,//already
// "media_embed": {},//maybe...
// "thumbnail_width": 140,//dont think so
// "author_flair_template_id": null,//dont think so
// "is_original_content": false,//already
// "user_reports": [],//maybe...
// "secure_media": null,//dont think so
// "is_reddit_media_domain": true,//already
// "is_meta": false,//already
// "category": null,//maybe...
// "secure_media_embed": {},//dont think so
// "link_flair_text": "Image",//dont think so
// "can_mod_post": false,//dont think so
// "score": 1,//already
// "approved_by": null,//dont think so
// "author_premium": false,//dont think so
// "thumbnail": "https://b.thumbs.redditmedia.com/KSOgyDVFIwdwOoRjTBuiNBCO6No23i09Yc5NXLJehVk.jpg",//already
// "edited": false,//dont think so
// "author_flair_css_class": null,//dont think so
// "author_flair_richtext": [],//dont think so
// "gildings": {},//dont think so
// "post_hint": "image",//dont think so
// "content_categories": null,//dont think so
// "is_self": false,//already
// "mod_note": null,//dont know that it is
// "created": 1620230469,//already
// "link_flair_type": "richtext",//dont think so
// "wls": 6,//dont know that it is
// "removed_by_category": null,//dont think so
// "banned_by": null,//dont think so
// "author_flair_type": "text",//dont think so
// "domain": "i.redd.it",//already
// "allow_live_comments": false,//already
// "selftext_html": null,//maybe
// "likes": null,//maybe
// "suggested_sort": null,//dont think so
// "banned_at_utc": null,//maybe
// "url_overridden_by_dest": "https://i.redd.it/so1xw3dpd9x61.png",//already
// "view_count": null,//maybe
// "archived": false,//already
// "no_follow": true,//dont think so
// "is_crosspostable": true,//already
// "pinned": false,//already
// "over_18": false,//already
// "preview": {//maybe but without auth or something i cannot reach image links
// "images": [
// {
// "source": {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?auto=webp&amp;s=9cc0a791c6ccd5b7fadc78cab3acfbb596527002",
// "width": 1023,
// "height": 750
// },
// "resolutions": [
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=108&amp;crop=smart&amp;auto=webp&amp;s=8451c1bf37cd073413dc69db8db6e93649544582",
// "width": 108,
// "height": 79
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=216&amp;crop=smart&amp;auto=webp&amp;s=0539def7ee7a7732d820912b13f834710803d3d5",
// "width": 216,
// "height": 158
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=320&amp;crop=smart&amp;auto=webp&amp;s=3057f38b5e1aa4367d826eedb81446dc41b38f1f",
// "width": 320,
// "height": 234
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=640&amp;crop=smart&amp;auto=webp&amp;s=93adfd16cf345366d0db1652d68447103433cf4a",
// "width": 640,
// "height": 469
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=960&amp;crop=smart&amp;auto=webp&amp;s=5eed04d3d0074fd8bdb1990baac8f124518837d1",
// "width": 960,
// "height": 703
// }
// ],
// "variants": {},
// "id": "aaJ1jfcNT4SwBGhciv45q_v9Wy3VchZY-UAP5pbc94c"
// }
// ],
// "enabled": true
// },
// "all_awardings": [],//dont think so
// "awarders": [],//dont think so
// "media_only": false,//already
// "can_gild": true,//dont think so
// "spoiler": false,//already
// "locked": false,//already
// "author_flair_text": null,//dont think so
// "treatment_tags": [],//dont think so
// "visited": false,//already
// "removed_by": null,//dont think so
// "num_reports": null,//dont think so
// "distinguished": null,//dont think so
// "subreddit_id": "t5_2rk5q",//already
// "mod_reason_by": null,//maybe
// "removal_reason": null,//maybe
// "link_flair_background_color": "",//dont think so
// "id": "n5anff",//already
// "is_robot_indexable": true,//dont think so
// "report_reasons": null,//maybe
// "author": "mochr91",//already
// "discussion_type": null,//dont think so
// "num_comments": 0,//already
// "send_replies": true,//already
// "whitelist_status": "all_ads",//dont think so
// "contest_mode": false,//maybe
// "mod_reports": [],//maybe
// "author_patreon_flair": false,//dont think so
// "author_flair_text_color": null,//dont think so
// "permalink": "/r/3Dprinting/comments/n5anff/experiment_load_coordinates_from_stl_binary_to/",//already
// "parent_whitelist_status": "all_ads",//dont think so
// "stickied": false,//already
// "url": "https://i.redd.it/so1xw3dpd9x61.png",//already
// "subreddit_subscribers": 700555,//already
// "created_utc": 1620201669,//already
// "num_crossposts": 0,//dont think so
// "media": null,//maybe
// "is_video": false//already
