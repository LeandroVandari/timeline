use core::ops::Deref;

use bevy::{
    ecs::{
        entity::EntityHashSet,
        query::{IterQueryData, QueryData, QueryFilter},
    },
    prelude::*,
};

pub trait QueryExt<'a, FN> {
    fn for_each_matching<D, F>(
        &'a self,
        entity: Entity,
        properties_query: &mut Query<'_, '_, D, F>,
        func: FN,
    ) where
        D: IterQueryData,
        F: QueryFilter,
        FN: FnMut(<D as QueryData>::Item<'_, '_>);
}

type QueryItem<'a, D> = <<D as QueryData>::ReadOnly as QueryData>::Item<'a, 'a>;

impl<'a, FN, DParent, FParent> QueryExt<'a, FN> for Query<'_, '_, DParent, FParent>
where
    DParent: QueryData,
    FParent: QueryFilter,
    QueryItem<'a, DParent>: Deref,
    <QueryItem<'a, DParent> as Deref>::Target: RelationshipTarget,
{
    #[inline]
    fn for_each_matching<D, F>(
        &'a self,
        entity: Entity,
        properties_query: &mut Query<'_, '_, D, F>,
        func: FN,
    ) where
        D: IterQueryData,
        F: QueryFilter,
        FN: FnMut(<D as QueryData>::Item<'_, '_>),
    {
        match self.get(entity) {
            Ok(collection) => {
                properties_query
                    .iter_many_unique_mut(EntityHashSet::from_iter(collection.iter()))
                    .for_each(func);
            }

            Err(bevy::ecs::query::QueryEntityError::QueryDoesNotMatch(_, _)) => {}
            Err(e) => error!("Error running query: {e}"),
        }
    }
}
