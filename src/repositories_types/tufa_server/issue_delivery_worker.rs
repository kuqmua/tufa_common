pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryExecuteTaskErrorNamed {
    DequeueTask {
        #[eo_error_occurence]
        dequeue_task: DequeueTaskErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    GetIssue {
        #[eo_error_occurence]
        get_issue: GetIssueErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DeleteTask {
        #[eo_error_occurence]
        delete_task: DeleteTaskErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DequeueTaskErrorNamed {
    PostgresPoolBegin {
        #[eo_display]
        postgres_pool_begin: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteTaskErrorNamed {
    PostgresDeleteTask {
        #[eo_display]
        postgres_delete_task: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    PostgresTransactionCommit {
        #[eo_display]
        postgres_transaction_commit: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub struct NewsletterIssue {
    pub title: std::string::String,
    pub text_content: std::string::String,
    pub html_content: std::string::String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetIssueErrorNamed {
    PostgresSelectNewsletterIssues {
        #[eo_display]
        postgres_select_newsletter_issues: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}
