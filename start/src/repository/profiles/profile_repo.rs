use mockall::automock;
use sqlx::{Error, query_as};
use async_trait::async_trait;
use crate::repository::repo::{DbRepo, EntityId, Repository};
use super::profile_model::NewProfile;
use super::profile_model::Profile;

#[automock]
#[async_trait]
pub trait ProfileDb {
    async fn insert_profile(&self, new_profile: NewProfile) -> Result<EntityId, Error>;
    async fn select_profile(&self, id: i64) -> Result<Option<Profile>, Error>;
}

pub struct ProfileRepo {
    repo: DbRepo
}

#[allow(unused)]
impl ProfileRepo {
    fn new(repo: DbRepo) -> Self {
        Self {
            repo
        }
    }
}

#[async_trait]
impl ProfileDb for ProfileRepo {
    async fn insert_profile(&self, new_profile: NewProfile) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(r"
            insert into Profile
            (user_name, full_name, description, region, main_url, avatar)
            values
            ($1, $2, $3, $4, $5, $6)
            returning id
        ")
            .bind(new_profile.user_name)
            .bind(new_profile.full_name)
            .bind(new_profile.description)
            .bind(new_profile.region)
            .bind(new_profile.main_url)
            .bind(new_profile.avatar)
            .fetch_one( self.repo.get_conn())
            .await
    }

    async fn select_profile(&self, id: i64) -> Result<Option<Profile>, Error> {
        query_as::<_, Profile>(r"
            select *
            from Profile
            where id = $1
        ")
        .bind(id)
        .fetch_optional(self.repo.get_conn())
        .await
    }
}

pub struct ProfileService<T: ProfileDb> {
    pub db: T
}
impl<T: ProfileDb> ProfileService<T> {
    pub fn new(db: T) -> Self {
        Self {
            db
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    use fake::faker::lorem::en::Sentence;
    use fake::{faker::internet::en::Username, Fake};
    use fake::faker::name::en::{FirstName, LastName};

    #[tokio::test]
    async fn test_insert_profile() {
        let mut mock_db = MockProfileDb::new();
        let user_name: String = Username().fake();
        let new_user_name = user_name.clone();
        let full_name = format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>());
        let new_full_name = full_name.clone();
        let description: String = Sentence(1..2).fake();
        let new_description = description.clone();
        let region = "usa".to_string();
        let new_region = region.clone();
        let main_url = "http://test.com".to_string();
        let new_main_url = main_url.clone();
        let avatar = vec![];
        let new_avatar = avatar.clone();
        
        mock_db
            .expect_insert_profile()
            .times(1)
            .returning(|_| Ok(EntityId { id: 1 }));
        mock_db
            .expect_select_profile()
            .times(1)
            .returning(move |_| Ok(
                Some(Profile {
                    id: 1,
                    user_name: user_name.clone(),
                    full_name: full_name.clone(),
                    description: description.clone(),
                    region: region.clone(),
                    main_url: main_url.clone(),
                    avatar: avatar.clone()
                })
            ));

        let service = ProfileService::new(mock_db);
        match service.db.insert_profile(NewProfile {
            user_name: new_user_name,
            full_name: new_full_name,
            description: new_description,
            region: new_region,
            main_url: new_main_url,
            avatar: new_avatar
        }).await {
            Ok(entity) => {
                let profile = service.db.select_profile(1).await.unwrap().unwrap();
                assert!(profile.id == entity.id);
            },
            Err(e) => panic!("{}", e.to_string())
        }
    }
}