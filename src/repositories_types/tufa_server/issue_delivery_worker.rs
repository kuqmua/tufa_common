pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum TryExecuteTaskErrorNamed<'a> {
    DequeueTask {
        #[eo_error_occurence]
        dequeue_task: DequeueTaskErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    GetIssue {
        #[eo_error_occurence]
        get_issue: GetIssueErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    DeleteTask {
        #[eo_error_occurence]
        delete_task: DeleteTaskErrorNamed<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DequeueTaskErrorNamed<'a> {
    PostgresPoolBegin {
        #[eo_display]
        postgres_pool_begin: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresSelect {
        #[eo_display]
        postgres_select: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum DeleteTaskErrorNamed<'a> {
    PostgresDeleteTask {
        #[eo_display]
        postgres_delete_task: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    PostgresTransactionCommit {
        #[eo_display]
        postgres_transaction_commit: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

pub struct NewsletterIssue {
    pub title: String,
    pub text_content: String,
    pub html_content: String,
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum GetIssueErrorNamed<'a> {
    PostgresSelectNewsletterIssues {
        #[eo_display]
        postgres_select_newsletter_issues: sqlx::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
