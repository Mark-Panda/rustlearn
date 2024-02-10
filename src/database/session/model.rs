use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::{types::time::OffsetDateTime, FromRow};
use uuid::{uuid, Uuid};

use crate::database::user::User;

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: uuid!("8147a9f8-2845-4f92-9e1d-0c0c6c8db79b"),
            user_id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            exp: OffsetDateTime::from(SystemTime::now()),
            user_agent: String::from("stub user agent"),
        }
    }
}

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
/// 这段代码定义了一个名为 DynSessionsRepository 的公共类型别名（type alias）。它是一个指向实现了 SessionsRepository trait 的类型的动态多态引用（dynamic polymorphic reference）。
// 让我们逐步解释这段代码的含义：
// pub: 这个关键字表示该类型别名是公共的，可以在模块外部访问。
// type: 这个关键字用于声明一个类型别名。
// DynSessionsRepository: 这是类型别名的名称，你可以根据需要选择一个有意义的名称。
// Arc: Arc 是 Rust 标准库中的一个类型，表示原子引用计数指针（atomic reference-counted pointer）。它允许多个所有者共享数据，并在不需要时自动释放。在这里，Arc 用于创建一个线程安全的指针，以便在多个线程之间共享 SessionsRepository 实例。
// dyn SessionsRepository: 这是一个动态多态类型的标记。它表示 SessionsRepository 是一个 trait（特征），而不是具体的类型。通过使用 dyn 关键字，我们可以在运行时选择具体的类型来实现多态性。
// + Send + Sync: 这是 trait bound（特质约束）的语法，指定了 SessionsRepository trait 的额外约束。Send 和 Sync 是 Rust 中的特质（trait），用于指定类型的可发送性（Sendability）和可同步性（Syncability）。Send 表示类型可以安全地在线程之间传递，而 Sync 表示类型可以安全地在多个线程之间共享访问。
// 因此，DynSessionsRepository 是一个公共类型别名，它指向一个实现了 SessionsRepository trait 的类型的动态多态引用。使用 Arc 来提供线程安全性，并添加了 Send 和 Sync 的约束以确保类型在多线程环境中的安全性。

pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsRepository {
    async fn new_session(
        &self,
        user_id: Uuid,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session>;

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}
