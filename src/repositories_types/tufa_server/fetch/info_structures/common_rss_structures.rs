#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, Default)] // PartialEq,//Default,
pub struct CommonRssPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<CommonRssPost>,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)] //PartialEq,//Default,
pub struct CommonRssPost {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    //meta
    // pub provider_kind: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
    //meta

    //arxiv specific
    //nothing here yet
    //arxiv specific

    //biorxiv specific
    pub biorxiv_date: Option<String>,
    pub biorxiv_identifier: Option<String>,
    pub biorxiv_publisher: Option<String>,
    pub biorxiv_publication_date: Option<String>,
    //biorxiv specific

    //github specific
    pub github_id: Option<String>,
    pub github_published: Option<String>,
    pub github_updated: Option<String>,
    pub github_media: Option<String>,
    pub github_author_uri: Option<String>,
    //github specific

    //habr specific
    pub habr_guid: Option<String>,
    pub habr_pub_date: Option<String>,
    pub habr_category: Option<Vec<String>>,
    //habr specific

    //medrxiv specific
    pub medrxiv_date: Option<String>,
    pub medrxiv_identifier: Option<String>,
    pub medrxiv_publisher: Option<String>,
    pub medrxiv_publication_date: Option<String>,
    //medrxiv specific

    //reddit specific
    // pub reddit_selftext: String,//into description feild
    pub reddit_url_overridden_by_dest: Option<String>,
    pub reddit_subreddit: Option<String>,
    pub reddit_id: Option<String>,
    pub reddit_author_fullname: Option<String>,
    pub reddit_domain: Option<String>,
    pub reddit_permalink: Option<String>,
    pub reddit_thumbnail: Option<String>,
    pub reddit_url: Option<String>,
    pub reddit_name: Option<String>,
    pub reddit_subreddit_id: Option<String>,
    pub reddit_subreddit_subscribers: Option<f64>,
    pub reddit_created: Option<f64>,
    pub reddit_upvote_ratio: Option<f64>,
    pub reddit_total_awards_received: Option<f64>,
    pub reddit_downs: Option<f64>,
    pub reddit_created_utc: Option<f64>,
    pub reddit_ups: Option<f64>,
    pub reddit_score: Option<f64>,
    pub reddit_num_comments: Option<u64>,
    pub reddit_is_video: Option<bool>,
    pub reddit_hidden: Option<bool>,
    pub reddit_send_replies: Option<bool>,
    pub reddit_stickied: Option<bool>,
    pub reddit_is_original_content: Option<bool>,
    pub reddit_is_reddit_media_domain: Option<bool>,
    pub reddit_is_meta: Option<bool>,
    pub reddit_allow_live_comments: Option<bool>,
    pub reddit_archived: Option<bool>,
    pub reddit_over_18: Option<bool>,
    pub reddit_quarantine: Option<bool>,
    pub reddit_is_self: Option<bool>,
    pub reddit_saved: Option<bool>,
    pub reddit_is_crosspostable: Option<bool>,
    pub reddit_pinned: Option<bool>,
    pub reddit_media_only: Option<bool>,
    pub reddit_spoiler: Option<bool>,
    pub reddit_locked: Option<bool>,
    pub reddit_visited: Option<bool>,
    //reddit specific

    //twitter specific
    pub twitter_pub_date: Option<String>,
    pub twitter_guid: Option<String>,
    //from TwitterStructForParsingImage
    pub twitter_author_link: Option<String>,
    //from TwitterStructForParsingImage
    //twitter specific
}
impl CommonRssPost {
    pub fn initialize_with_params(
        title: Option<String>,
        link: Option<String>,
        description: Option<String>,
        creator: Option<String>,
        //meta
        // provider_kind: crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
        //meta

        //arxiv specific
        //nothing here yet
        //arxiv specific

        //biorxiv specific
        biorxiv_date: Option<String>,
        biorxiv_identifier: Option<String>,
        biorxiv_publisher: Option<String>,
        biorxiv_publication_date: Option<String>,
        //biorxiv specific

        //github specific
        github_id: Option<String>,
        github_published: Option<String>,
        github_updated: Option<String>,
        github_media: Option<String>,
        github_author_uri: Option<String>,
        //github specific

        //habr specific
        habr_guid: Option<String>,
        habr_pub_date: Option<String>,
        habr_category: Option<Vec<String>>,
        //habr specific

        //medrxiv specific
        medrxiv_date: Option<String>,
        medrxiv_identifier: Option<String>,
        medrxiv_publisher: Option<String>,
        medrxiv_publication_date: Option<String>,
        //medrxiv specific

        //reddit specific
        // reddit_selftext: String,//into description feild
        reddit_url_overridden_by_dest: Option<String>,
        reddit_subreddit: Option<String>,
        reddit_id: Option<String>,
        reddit_author_fullname: Option<String>,
        reddit_domain: Option<String>,
        reddit_permalink: Option<String>,
        reddit_thumbnail: Option<String>,
        reddit_url: Option<String>,
        reddit_name: Option<String>,
        reddit_subreddit_id: Option<String>,
        reddit_subreddit_subscribers: Option<f64>,
        reddit_created: Option<f64>,
        reddit_upvote_ratio: Option<f64>,
        reddit_total_awards_received: Option<f64>,
        reddit_downs: Option<f64>,
        reddit_created_utc: Option<f64>,
        reddit_ups: Option<f64>,
        reddit_score: Option<f64>,
        reddit_num_comments: Option<u64>,
        reddit_is_video: Option<bool>,
        reddit_hidden: Option<bool>,
        reddit_send_replies: Option<bool>,
        reddit_stickied: Option<bool>,
        reddit_is_original_content: Option<bool>,
        reddit_is_reddit_media_domain: Option<bool>,
        reddit_is_meta: Option<bool>,
        reddit_allow_live_comments: Option<bool>,
        reddit_archived: Option<bool>,
        reddit_over_18: Option<bool>,
        reddit_quarantine: Option<bool>,
        reddit_is_self: Option<bool>,
        reddit_saved: Option<bool>,
        reddit_is_crosspostable: Option<bool>,
        reddit_pinned: Option<bool>,
        reddit_media_only: Option<bool>,
        reddit_spoiler: Option<bool>,
        reddit_locked: Option<bool>,
        reddit_visited: Option<bool>,
        //reddit specific

        //twitter specific
        twitter_pub_date: Option<String>,
        twitter_guid: Option<String>,
        //from TwitterStructForParsingImage
        twitter_author_link: Option<String>,
        //from TwitterStructForParsingImage
        //twitter specific
    ) -> Self {
        CommonRssPost {
            title,
            link,
            description,
            creator,
            //meta
            // provider_kind,
            //meta

            //arxiv specific
            //nothing here yet
            //arxiv specific

            //biorxiv specific
            biorxiv_date,
            biorxiv_identifier,
            biorxiv_publisher,
            biorxiv_publication_date,
            //biorxiv specific

            //github specific
            github_id,
            github_published,
            github_updated,
            github_media,
            github_author_uri,
            //github specific

            //habr specific
            habr_guid,
            habr_pub_date,
            habr_category,
            //habr specific

            //medrxiv specific
            medrxiv_date,
            medrxiv_identifier,
            medrxiv_publisher,
            medrxiv_publication_date,
            //medrxiv specific

            //reddit specific
            // selftext: String,//into description feild
            reddit_url_overridden_by_dest,
            reddit_subreddit,
            reddit_id,
            reddit_author_fullname,
            reddit_domain,
            reddit_permalink,
            reddit_thumbnail,
            reddit_url,
            reddit_name,
            reddit_subreddit_id,
            reddit_subreddit_subscribers,
            reddit_created,
            reddit_upvote_ratio,
            reddit_total_awards_received,
            reddit_downs,
            reddit_created_utc,
            reddit_ups,
            reddit_score,
            reddit_num_comments,
            reddit_is_video,
            reddit_hidden,
            reddit_send_replies,
            reddit_stickied,
            reddit_is_original_content,
            reddit_is_reddit_media_domain,
            reddit_is_meta,
            reddit_allow_live_comments,
            reddit_archived,
            reddit_over_18,
            reddit_quarantine,
            reddit_is_self,
            reddit_saved,
            reddit_is_crosspostable,
            reddit_pinned,
            reddit_media_only,
            reddit_spoiler,
            reddit_locked,
            reddit_visited,
            //reddit specific

            //twitter specific
            twitter_pub_date,
            twitter_guid,
            //from TwitterStructForParsingImage
            twitter_author_link,
            //from TwitterStructForParsingImage
            //twitter specific
        }
    }
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GithubInfoFromHtml {
    pub avatar_link: Option<String>,
    pub author: Option<String>,
    pub action: Option<String>,
    pub repository: Option<String>,
    pub from_what_repository_forked: Option<String>,
    pub from: Option<String>,
    pub datejs: Option<String>,
    pub date: Option<String>,
    pub actionto: Option<String>,
    pub branch: Option<String>,
    pub release_tag: Option<String>,
    pub of: Option<String>,
    pub bot_tag: Option<String>,
    pub who_follow: Option<String>,
}

impl GithubInfoFromHtml {
    pub fn initialize_with_params(
        avatar_link: Option<String>,
        author: Option<String>,
        action: Option<String>,
        repository: Option<String>,
        from_what_repository_forked: Option<String>,
        from: Option<String>,
        datejs: Option<String>,
        date: Option<String>,
        actionto: Option<String>,
        branch: Option<String>,
        release_tag: Option<String>,
        of: Option<String>,
        bot_tag: Option<String>,
        who_follow: Option<String>,
    ) -> Self {
        GithubInfoFromHtml {
            avatar_link,
            author,
            action,
            repository,
            from_what_repository_forked,
            from,
            datejs,
            date,
            actionto,
            branch,
            release_tag,
            of,
            bot_tag,
            who_follow,
        }
    }
}
