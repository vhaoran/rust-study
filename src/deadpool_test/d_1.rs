
#[cfg(feature = "managed")]
mod tests {
    use std::time::Duration;
    use deadpool::managed::{Object, Pool, RecycleResult};
    use tokio::time::sleep;

    use async_trait::async_trait;

    struct Manager {}

    #[async_trait]
    impl deadpool::managed::Manager<usize, ()> for Manager {
        async fn create(&self) -> Result<usize, ()> {
            Ok(0)
        }
        async fn recycle(&self, _conn: &mut usize) -> RecycleResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_managed_basic() {
        let mgr = Manager {};
        let pool = Pool::new(mgr, 16);

        let status = pool.status();
        assert_eq!(status.size, 0);
        assert_eq!(status.available, 0);
        println!("----d_1.rs-------{:?}-" ,status);

        let obj0 = pool.get().await.unwrap();
        let status = pool.status();
        println!("----d_1.rs---aa----{:?}-" ,status);
        assert_eq!(status.size, 1);
        assert_eq!(status.available, 0);

        let obj1 = pool.get().await.unwrap();
        let status = pool.status();
        assert_eq!(status.size, 2);
        assert_eq!(status.available, 0);

        let obj2 = pool.get().await.unwrap();
        let status = pool.status();
        assert_eq!(status.size, 3);
        assert_eq!(status.available, 0);

        drop(obj0);
        let status = pool.status();
        assert_eq!(status.size, 3);
        assert_eq!(status.available, 1);

        drop(obj1);
        let status = pool.status();
        assert_eq!(status.size, 3);
        assert_eq!(status.available, 2);

        drop(obj2);
        let status = pool.status();
        assert_eq!(status.size, 3);
        assert_eq!(status.available, 3);
    }

}