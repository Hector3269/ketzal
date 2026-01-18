use super::route_data::RouteData;

pub struct RouteRegistry {
    routes: Vec<RouteData>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn register(&mut self, route: RouteData) {
        self.routes.push(route);
    }

    pub fn get_routes(&self) -> &Vec<RouteData> {
        &self.routes
    }
}
