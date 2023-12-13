use hecs::Entity;



struct Component;


pub struct ECSQueries<'a, 'c> {
    query1: InstancesQuery<'a, 'c>,
}

impl<'a, 'c: 'a> ECSQueries<'a, 'c> {
    pub fn new(world: &'c hecs::World) -> Self {

        Self {
            query1: world.query(),
        }
    }
}

pub struct ECSViews<'b, 'c> {
    view1: InstancesView<'b, 'c>,
}

impl<'a, 'b, 'c> ECSViews<'b, 'c> {
    pub fn new(queries: &'b mut ECSQueries<'a, 'c>) -> Self {

        Self {
            view1: queries.query1.view(),
        }
    }
}


type InstancesQuery<'a, 'c> = hecs::QueryBorrow<'a, (&'c Component,)>;
type InstancesView<'b, 'c> = hecs::View<'b, (&'c Component,)>;


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