use hecs::Entity;



struct Component;


pub struct ECSQueries<'a, 'b> {
    query1: InstancesQuery<'a, 'b>,
}

impl<'a, 'b, 'c: 'a> ECSQueries<'a, 'b> {
    pub fn new(world: &'c hecs::World) -> Self {

        Self {
            query1: world.query(),
        }
    }
}

pub struct ECSViews<'a, 'b> {
    view1: InstancesView<'a, 'b>,
}

impl<'a, 'b: 'a, 'c: 'a> ECSViews<'a, 'b> {
    pub fn new(queries: &'c mut ECSQueries<'a, 'b>) -> Self {

        Self {
            view1: queries.query1.view(),
        }
    }
}


pub type InstancesQuery<'a, 'b> = hecs::QueryBorrow<'a, (&'b Component,)>;
pub type InstancesView<'a, 'b> = hecs::View<'a, (&'b Component,)>;


fn main() {

    let world = hecs::World::new();


    //this works
    {
        let mut query1: InstancesQuery = world.query();
        let view_1: InstancesView = query1.view();

        let _ = view_1.get(Entity::DANGLING);
    }
    
    //this doesn't
    {
        let mut ecs_queries = ECSQueries::new(&world);
        let ecs_views = ECSViews::new(&mut ecs_queries);
        
        let _ = ecs_views.view1.get(Entity::DANGLING);
    }
}